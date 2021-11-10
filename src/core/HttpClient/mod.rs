use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use hyper::{client::ResponseFuture, Body, Client, Error, Request};

use crate::util::Requests::get_header_as;

mod RequestBucket;
mod RequestFuture;
mod RequestQueue;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct RequestRoute {
    base_route: String,
    major_param: String,
}

pub struct HttpSchedulerClient {
    send_queue: Arc<Mutex<RequestQueue::Queue>>,
}

impl HttpSchedulerClient {
    pub fn new() -> HttpSchedulerClient {
        HttpSchedulerClient {
            send_queue: Arc::new(Mutex::new(RequestQueue::Queue::new())),
        }
    }

    pub fn spawn_req_thread(&mut self) {
        let send_queue = self.send_queue.clone();
        unsafe impl Send for RequestQueue::Queue {}

        thread::spawn(move || {
            let client = Client::new();
            let global_limit = 1;

            // TODO: Clean the buckets at certain times
            let buckets: HashMap<RequestRoute, RequestBucket::Bucket> = HashMap::new();

            loop {
                let Routes = {
                    let locked = send_queue.lock().unwrap();
                    let reqs = locked.active_requests_queue.clone();
                };
                let mut queue_mut = {
                    send_queue
                        .lock()
                        .unwrap()
                        .queue_map
                        .get_mut(&route)
                        .unwrap()
                };

                for route in Routes {
                    // TODO Break out if hit global rate limit

                    let now = chrono::Utc::now().timestamp();

                    let bucket = buckets
                        .get_mut(&route)
                        .or_else(|| {
                            let mut bucket = RequestBucket::Bucket::new();
                            buckets.insert(route.clone(), bucket);
                            Some(&mut bucket)
                        })
                        .unwrap();

                    if bucket.reset_at < now {
                        bucket.remaining_requests = bucket.max_requests;
                    }

                    if bucket.remaining_requests == 0 {
                        continue;
                    }

                    let queue = queue_mut.lock().unwrap();

                    let responses: Vec<(&mut RequestFuture::ReqFuture, ResponseFuture)> =
                        Vec::new();

                    while bucket.remaining_requests > 0 {
                        let mut req_raw = match queue.pop_front() {
                            Some(req_raw) => req_raw,
                            None => break,
                        };
                        let req_future = unsafe { &mut *req_raw };
                        bucket.remaining_requests -= 1;

                        let req = {
                            let shared_state = req_future.shared_state.lock();
                            client.request(shared_state.unwrap().request)
                        };
                        responses.push((req_future, req));
                    }

                    async {
                        let last_date: i64 = 0;
                        for (req, future) in responses {
                            let received = future.await;
                            let date_raw =
                                received.unwrap().headers().get("Date").unwrap().as_bytes();
                            let date = chrono::DateTime::parse_from_rfc2822(
                                &std::str::from_utf8(&date_raw).unwrap(),
                            )
                            .unwrap()
                            .timestamp();

                            if date > last_date {
                                last_date = date;
                                bucket.remaining_requests = get_header_as::<i32>(
                                    received.unwrap().headers(),
                                    "X-RateLimit-Remaining",
                                )
                                .unwrap();

                                bucket.max_requests = get_header_as::<i32>(
                                    received.unwrap().headers(),
                                    "X-RateLimit-Limit",
                                )
                                .unwrap();

                                bucket.reset_at = get_header_as::<i64>(
                                    received.unwrap().headers(),
                                    "X-RateLimit-Reset",
                                )
                                .unwrap();
                            }

                            let shared_state = req.shared_state.lock();
                            shared_state.unwrap().response = Some(received);
                            match shared_state.unwrap().waker {
                                Some(waker) => waker.wake(),
                                None => (),
                            }
                        }
                    };
                }
            }
        });
    }

    pub async fn send_request(
        &self,
        route: RequestRoute,
        request: Request<Body>,
    ) -> Result<hyper::Response<Body>, Error> {
        let future = RequestFuture::ReqFuture::new(request);
        // Maybe use req_thread.unpark() to reduce cpu load while the thread is waiting for requests.
        // This would have the downside of increasing the power required make a request since we have to attempt to unpark it every time.
        // We could maybe get around this by having a parked flag, but this would require a mutex which also increases the power required.

        unsafe {
            let deq: &mut Mutex<RequestQueue::Queue>;
            match self.send_queue.lock() {
                Ok(mut to_send) => to_send.push(route, &future as *const _),
                Err(e) => {
                    panic!("{}", e);
                }
            };
        }
        future.await
    }
}
