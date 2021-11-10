use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use async_std;
use hyper::{client::ResponseFuture, Body, Client, Error, Request};

use crate::util::Requests::get_header_as;

mod request_bucket;
mod request_future;
mod request_queue;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct RequestRoute {
    pub base_route: String,
    pub major_param: String,
}

pub struct HttpSchedulerClient {
    send_queue: Arc<Mutex<request_queue::Queue>>,
}

impl Default for HttpSchedulerClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpSchedulerClient {
    pub fn new() -> HttpSchedulerClient {
        HttpSchedulerClient {
            send_queue: Arc::new(Mutex::new(request_queue::Queue::new())),
        }
    }

    pub fn spawn_req_thread(&mut self) {
        let send_queue = self.send_queue.clone();
        unsafe impl Send for request_queue::Queue {}

        thread::Builder::new()
            .name("Request_Thread".to_string())
            .spawn(move || {
                let client = Client::new();
                let mut global_limit = 1;

                // TODO: Clean the buckets at certain times
                let mut buckets: HashMap<RequestRoute, request_bucket::Bucket> = HashMap::new();

                loop {
                    let mut futures: Vec<(RequestRoute, &mut request_future::ReqFuture)> =
                        Vec::new();
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

                    let responses: Vec<(
                        RequestRoute,
                        &mut request_future::ReqFuture,
                        ResponseFuture,
                    )> = futures
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

                                    bucket.max_requests = get_header_as::<i32>(
                                        received.headers(),
                                        "X-RateLimit-Limit",
                                    )
                                    .unwrap_or(1);

                                    bucket.reset_at = get_header_as::<i64>(
                                        received.headers(),
                                        "X-RateLimit-Reset",
                                    )
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

    pub async fn send_request(
        &self,
        route: RequestRoute,
        request: Request<Body>,
    ) -> Result<hyper::Response<Body>, Error> {
        let mut future = request_future::ReqFuture::new(request);
        // Maybe use req_thread.unpark() to reduce cpu load while the thread is waiting for requests.
        // This would have the downside of increasing the power required make a request since we have to attempt to unpark it every time.
        // We could maybe get around this by having a parked flag, but this would require a mutex which also increases the power required.

        match self.send_queue.lock() {
            Ok(mut to_send) => to_send.push(&route, &mut future as *mut _),
            Err(e) => {
                panic!("{}", e);
            }
        };

        future.await
    }
}
