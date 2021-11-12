use std::{collections::HashMap, thread, time::Instant};

use crossbeam_channel::Receiver;
use hyper::{client::ResponseFuture, Client};

use crate::util::requests::get_header_as;

use super::{
    rate_limit_client::{RequestObject, RequestRoute},
    request_bucket,
    request_future::{self},
    request_queue::HttpQueue,
};

const GLOBAL_RATE_LIMIT_PER_SEC: f64 = 50f64;
const CLEAN_EVERY_N_REQUESTS: u64 = 10_000;

/**
 * Creates the request thread that will batch requests out according to rate limit headers that are returned by discord, and also the
 * Global rate limit of GLOBAL_RATE_LIMIT_PER_SEC
 * @param send_queue The Shared Queue that requests can be added to
 */
pub fn create_thread<T>(mut http_queue: T, receiver: Receiver<RequestObject>)
where
    T: HttpQueue + Send + 'static,
{
    thread::Builder::new()
        .name("Request_Thread".to_string())
        .spawn(move || {
            let client = Client::new();
            let mut global_allowance: f64 = GLOBAL_RATE_LIMIT_PER_SEC;
            let mut last_timestamp = Instant::now();
            let mut requests_sent: u64 = 0;

            // TODO: Clean the buckets at certain times, also clean the send_queue so that the hashmap doesn't continuously grow in size
            let mut rate_buckets: HashMap<RequestRoute, request_bucket::Bucket> = HashMap::new();

            // Main Request Loop
            loop {
                // Add incoming requests to the queue
                while !receiver.is_empty() {
                    let obj = receiver.recv().unwrap();
                    http_queue.push(&obj.route, obj.future);
                }

                // TODO Figure out a smarter way to do this
                // // check if we should clean the queue, and the buckets
                // if requests_sent % CLEAN_EVERY_N_REQUESTS == 0 {
                //     http_queue.clean();
                // }

                // Add more allowance to the global limit
                let temp_time = Instant::now();
                global_allowance += temp_time.duration_since(last_timestamp).as_secs_f64()
                    * GLOBAL_RATE_LIMIT_PER_SEC;

                if global_allowance > GLOBAL_RATE_LIMIT_PER_SEC {
                    global_allowance = GLOBAL_RATE_LIMIT_PER_SEC;
                }

                last_timestamp = Instant::now();

                let sorted_routes = http_queue.get_sorted_requests();

                let mut responses: Vec<(
                    RequestRoute,
                    &mut request_future::HttpFuture,
                    ResponseFuture,
                )> = Vec::new();

                // Iterate through all of the requests in the queue, and add them to the futures vector if they can be executed
                for route in sorted_routes {
                    // Get the bucket for this route, or create it if it doesn't exist
                    let bucket = match rate_buckets.get_mut(&route) {
                        Some(bucket) => bucket,
                        None => {
                            let new_bucket = request_bucket::Bucket::new();
                            rate_buckets.insert(route.clone(), new_bucket);
                            rate_buckets.get_mut(&route).unwrap()
                        }
                    };

                    // Reset the bucket if it is past the reset time
                    if bucket.reset_at < chrono::Utc::now().timestamp() {
                        bucket.remaining_requests = bucket.max_requests;
                    }

                    // get the queue for the route, and then get as many requests as possible from the queue
                    // This means it will take min(global_limit, bucket.remaining_requests) requests from the queue
                    let queue = http_queue.get_bucket_queue(&route).unwrap();
                    while bucket.remaining_requests > 0 && global_allowance >= 1f64 {
                        // Pop the front and add it to the futures vector if it exists, or break out if the queue is empty
                        match queue.pop() {
                            Some((_, req_future)) => {
                                let future_ptr = unsafe { &mut *req_future };

                                let req = {
                                    let mut shared_state = future_ptr.shared_state.lock().unwrap();
                                    client.request(shared_state.request.take().unwrap())
                                };
                                responses.push((route.clone(), future_ptr, req));
                                requests_sent += 1;

                                bucket.remaining_requests -= 1;
                                global_allowance -= 1f64;
                            }
                            None => {
                                break;
                            }
                        }
                    }
                    if queue.is_empty() {
                        http_queue.notify_empty(&route);
                    }
                    if global_allowance < 1f64 {
                        break;
                    }
                }

                // Convert the requests into a vector of response futures by having the hyper client make them

                let mut last_date_map: HashMap<RequestRoute, i64> = HashMap::new();

                // Collect the responses, and resolve all of the Request Futures
                for (route, req, future) in responses {
                    // Block execution until the future is resolved, and then process the rate limit information from the response
                    let receives = match async_std::task::block_on(future) {
                        Ok(received) => {
                            // Get the date of the response execution so that we know the last time the route was used,
                            // And therefore the most up to date rate limit information for each route
                            let date_raw = received.headers().get("Date").unwrap().as_bytes();
                            let date = chrono::DateTime::parse_from_rfc2822(
                                std::str::from_utf8(date_raw).unwrap(),
                            )
                            .unwrap()
                            .timestamp();

                            // Only update rate limit information if this request is more recent than the rest
                            if date > *last_date_map.get(&route).or(Some(&0)).unwrap() {
                                last_date_map.insert(route.clone(), date);
                                let bucket = rate_buckets.get_mut(&route).unwrap();
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
