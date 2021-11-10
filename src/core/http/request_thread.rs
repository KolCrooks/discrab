use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use hyper::{client::ResponseFuture, Client};

use crate::util::Requests::get_header_as;

use super::{request_bucket, request_future, request_queue::Queue, RequestRoute};

pub fn create_thread(send_queue: Arc<Mutex<Queue>>) {
    thread::Builder::new()
        .name("Request_Thread".to_string())
        .spawn(move || {
            let client = Client::new();
            let mut global_limit = 1; // TODO Implement global limit reset timer

            // TODO: Clean the buckets at certain times, also clean the send_queue so that the hashmap doesn't continuously grow in size
            let mut buckets: HashMap<RequestRoute, request_bucket::Bucket> = HashMap::new();

            loop {
                let mut futures: Vec<(RequestRoute, &mut request_future::ReqFuture)> = Vec::new();
                {
                    let mut locked = send_queue.lock().unwrap();
                    let reqs = locked.active_requests_queue.clone();
                    let mut i = 0;
                    let mut to_remove: Vec<i32> = Vec::new();

                    // Go through each request that needs to be sent
                    for req in reqs {
                        let route = req.clone();
                        let bucket = match buckets.get_mut(&route) {
                            Some(bucket) => bucket,
                            None => {
                                let new_bucket = request_bucket::Bucket::new();
                                buckets.insert(route.clone(), new_bucket);
                                buckets.get_mut(&route).unwrap()
                            }
                        };

                        if bucket.reset_at < chrono::Utc::now().timestamp() {
                            bucket.remaining_requests = bucket.max_requests;
                        }

                        let queue = locked.queue_map.get_mut(&route).unwrap();
                        while bucket.remaining_requests > 0 {
                            if global_limit == 0 {
                                break;
                            }
                            match queue.get_mut().unwrap().pop_front() {
                                Some(req_future) => {
                                    futures.push((req.clone(), unsafe { &mut *req_future }));
                                    bucket.remaining_requests -= 1;
                                    global_limit -= 1;
                                }
                                None => {
                                    to_remove.push(i);
                                    i -= 1;
                                }
                            }
                        }
                        if global_limit == 0 {
                            break;
                        }
                        i += 1;
                    }
                    for i in to_remove {
                        let route = locked.active_requests_queue.remove(i as usize).unwrap();
                        locked.active_requests_set.remove(&route);
                    }
                };

                let responses: Vec<(RequestRoute, &mut request_future::ReqFuture, ResponseFuture)> =
                    futures
                        .into_iter()
                        .map(|(route, req_future)| {
                            let req = {
                                let mut shared_state = req_future.shared_state.lock().unwrap();
                                client.request(shared_state.request.take().unwrap())
                            };
                            (route, req_future, req)
                        })
                        .collect();

                let mut last_date: HashMap<RequestRoute, i64> = HashMap::new();

                for (route, req, future) in responses {
                    let receives = match async_std::task::block_on(future) {
                        Ok(received) => {
                            let date_raw = received.headers().get("Date").unwrap().as_bytes();
                            let date = chrono::DateTime::parse_from_rfc2822(
                                std::str::from_utf8(date_raw).unwrap(),
                            )
                            .unwrap()
                            .timestamp();

                            if date > *last_date.get(&route).or(Some(&0)).unwrap() {
                                last_date.insert(route.clone(), date);
                                let bucket = buckets.get_mut(&route).unwrap();
                                bucket.remaining_requests = get_header_as::<i32>(
                                    received.headers(),
                                    "X-RateLimit-Remaining",
                                )
                                .unwrap_or(0);

                                bucket.max_requests =
                                    get_header_as::<i32>(received.headers(), "X-RateLimit-Limit")
                                        .unwrap_or(1);

                                bucket.reset_at =
                                    get_header_as::<i64>(received.headers(), "X-RateLimit-Reset")
                                        .unwrap_or(0); // TODO make this an actual value
                            }
                            Ok(received)
                        }
                        Err(e) => Err(e),
                    };

                    let mut shared_state = req.shared_state.lock().unwrap();
                    shared_state.commit(receives);
                }
            }
        })
        .unwrap();
}
