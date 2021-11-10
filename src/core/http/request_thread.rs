use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use hyper::{client::ResponseFuture, Client};

use crate::util::Requests::get_header_as;

use super::{request_bucket, request_future, request_queue::Queue, RequestRoute};

const GLOBAL_RATE_LIMIT_PER_SEC: u64 = 50;

/**
 * Creates the request thread that will batch requests out according to rate limit headers that are returned by discord, and also the
 * Global rate limit of 50
 * @param send_queue The Shared Queue that requests can be added to
 */
pub fn create_thread(send_queue: Arc<Mutex<Queue>>) {
    thread::Builder::new()
        .name("Request_Thread".to_string())
        .spawn(move || {
            let client = Client::new();
            let mut global_allowance: u64 = GLOBAL_RATE_LIMIT_PER_SEC;
            let mut last_timestamp = Instant::now();

            // TODO: Clean the buckets at certain times, also clean the send_queue so that the hashmap doesn't continuously grow in size
            let mut buckets: HashMap<RequestRoute, request_bucket::Bucket> = HashMap::new();

            // Main Request Loop
            loop {
                // Add more allowance to the global limit
                let temp_time = Instant::now();
                global_allowance += (temp_time.duration_since(last_timestamp).as_secs_f32()
                    * GLOBAL_RATE_LIMIT_PER_SEC as f32) as u64;

                if global_allowance > GLOBAL_RATE_LIMIT_PER_SEC {
                    global_allowance = GLOBAL_RATE_LIMIT_PER_SEC;
                }

                last_timestamp = temp_time;

                let futures: Vec<(RequestRoute, &mut request_future::HttpFuture)> =
                    get_requests(&send_queue, &mut buckets, &mut global_allowance);

                // Convert the requests into a vector of response futures by having the hyper client make them
                let responses: Vec<(
                    RequestRoute,
                    &mut request_future::HttpFuture,
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

/**
 * Get as many requests from the queue as possible. This means that it will obey both the global rate limit,
 *  and the individual rate limit for each route (bucket). It will try to prioritize older requests.
 *
 * @param send_queue The queue containing the requests to be sent
 * @param buckets Buckets map to reference for rate limit information for each route
 * @param global_allowance The global rate limit allowance
 * @return A vector of futures that can be be sent without violating the rate limit.
 */
fn get_requests<'a>(
    send_queue: &Arc<Mutex<Queue>>,
    buckets: &mut HashMap<RequestRoute, request_bucket::Bucket>,
    global_allowance: &mut u64,
) -> Vec<(RequestRoute, &'a mut request_future::HttpFuture)> {
    let mut futures = Vec::new();
    let mut locked = send_queue.lock().unwrap();

    // TODO this implementation technically doesn't guarantee that the requests will be sent in the order they were added
    // It only gets called in the order that the request routes were added, so if someone adds a request route that is already
    // low on the list, it will skip the waitlist and get grouped in with older requests. This is fine for now, but it would be
    // better if we made this in a way that guarantees that the requests are sent in (about) the order they were added.
    let requests = locked.active_requests_queue.clone();

    let mut i = 0;
    let mut to_remove: Vec<i32> = Vec::new();

    // Iterate through all of the requests in the queue, and add them to the futures vector if they can be executed
    for req in requests {
        let route = req.clone();

        // Get the bucket for this route, or create it if it doesn't exist
        let bucket = match buckets.get_mut(&route) {
            Some(bucket) => bucket,
            None => {
                let new_bucket = request_bucket::Bucket::new();
                buckets.insert(route.clone(), new_bucket);
                buckets.get_mut(&route).unwrap()
            }
        };

        // Reset the bucket if it is past the reset time
        if bucket.reset_at < chrono::Utc::now().timestamp() {
            bucket.remaining_requests = bucket.max_requests;
        }

        // get the queue for the route, and then get as many requests as possible from the queue
        // This means it will take min(global_limit, bucket.remaining_requests) requests from the queue
        let queue = locked.queue_map.get_mut(&route).unwrap();
        while bucket.remaining_requests > 0 && *global_allowance != 0 {
            // Pop the front and add it to the futures vector if it exists, or break out if the queue is empty
            match queue.get_mut().unwrap().pop_front() {
                Some(req_future) => {
                    futures.push((req.clone(), unsafe { &mut *req_future }));
                    bucket.remaining_requests -= 1;
                    *global_allowance -= 1;
                }
                None => {
                    to_remove.push(i);
                    i -= 1;
                    break;
                }
            }
        }
        if *global_allowance == 0 {
            break;
        }
        i += 1;
    }

    // Remove the empty queues from the active requests set and queue because it means there are no more queued requests for that route
    for i in to_remove {
        let route = locked.active_requests_queue.remove(i as usize).unwrap();
        locked.active_requests_set.remove(&route);
    }
    futures
}
