use std::{
    collections::{HashMap, HashSet, LinkedList},
    time::Instant,
};

use super::{rate_limit_client::RequestRoute, request_future};

/// This is a generic queue that supplies the http client with requests in a given order as designated by the queue.
pub trait HttpQueue {
    /**
     * Add a request to the queue
     * @param route The route of the request
     * @param future The request
     */
    fn push(&mut self, route: &RequestRoute, future: *mut request_future::HttpFuture);
    /// Get the requests as sorted by the queue
    fn get_sorted_requests(&self) -> Vec<RequestRoute>;
    /// Get the queue for a given route
    fn get_bucket_queue(&mut self, route: &RequestRoute) -> Option<&mut BucketQueue>;
    /// This function is called when the bucket for a given route becomes empty
    fn notify_empty(&mut self, route: &RequestRoute);
    /// This function is called periodically, and should be used to clean up any expired buckets
    fn clean(&mut self);
    /// This function should return if the queue is empty
    fn is_empty(&self) -> bool;
}

/// Queue of requests to be made for a given bucket. This allows for unique ordering of requests that prioritize different things.
pub struct BucketQueue {
    /// The time that the bucket became empty
    time_of_empty: Instant,
    /// The queue of requests
    queue: LinkedList<(u64, *mut request_future::HttpFuture)>,
}

impl BucketQueue {
    pub fn new() -> BucketQueue {
        BucketQueue {
            time_of_empty: Instant::now(),
            queue: LinkedList::new(),
        }
    }

    /**
     * Add a request to the queue.
     * @param time The time that the request was added
     * @param future The request future
     */
    pub fn push(&mut self, time: u64, future: *mut request_future::HttpFuture) {
        self.queue.push_back((time, future));
    }

    /// Get the oldest request in the queue
    pub fn get_oldest(&self) -> Option<&(u64, *mut request_future::HttpFuture)> {
        self.queue.front()
    }

    /// Removes the first request in the queue, and returns the request
    pub fn pop(&mut self) -> Option<(u64, *mut request_future::HttpFuture)> {
        self.queue.pop_front()
    }

    /// Returns true if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns the time that the bucket became empty
    pub fn get_time_of_empty(&self) -> Instant {
        self.time_of_empty
    }
}
impl Default for BucketQueue {
    fn default() -> Self {
        BucketQueue::new()
    }
}

/// A queue of all requests that are waiting to be made.
/// It is a basic implementation that prioritizes requests based on the time
pub struct BasicHttpQueue {
    /// This field is used to give a unique request id to each request by incrementing it after each request is added
    req_id_cnt: u64,
    /// If a bucket is empty for a period of time, it will be removed from this map
    inactive_bucket_timeout: u64,
    /// The map of buckets to queues
    queue_map: HashMap<RequestRoute, BucketQueue>,

    active_requests_set: HashSet<RequestRoute>,
}
unsafe impl Send for BasicHttpQueue {}

impl BasicHttpQueue {
    /**
     * @param inactive_bucket_timeout: the time in seconds after which a bucket is considered inactive, and can be deleted
     */
    pub fn new(inactive_bucket_timeout: u64) -> BasicHttpQueue {
        BasicHttpQueue {
            req_id_cnt: 0,
            inactive_bucket_timeout,
            queue_map: HashMap::new(),
            active_requests_set: HashSet::new(),
        }
    }
}

impl HttpQueue for BasicHttpQueue {
    /// Add a request to the queue
    fn push(&mut self, route: &RequestRoute, future: *mut request_future::HttpFuture) {
        let queue = self
            .queue_map
            .entry(route.clone())
            .or_insert_with(BucketQueue::new);

        queue.push(self.req_id_cnt, future);
        self.req_id_cnt += 1;
        self.active_requests_set.insert(route.clone());
    }

    /**
     * Gets the request groups in order by the first item's age. This will prioritize
     * Requests with older requests, but will not mean that all requests will be processed in order
     */
    fn get_sorted_requests(&self) -> Vec<RequestRoute> {
        // TODO: Probably not the fastest way to do this since it is running O(n) and then O(nlogn)
        // You could probably do this with a single O(nlogn)
        let mut q: Vec<RequestRoute> = self.active_requests_set.clone().into_iter().collect();

        q.sort_by(|a, b| {
            let a_time = self.queue_map.get(a).unwrap().get_oldest().unwrap().0;
            let b_time = self.queue_map.get(b).unwrap().get_oldest().unwrap().0;

            a_time.cmp(&b_time)
        });
        q
    }

    fn clean(&mut self) {
        self.queue_map.retain(|_, v| {
            v.is_empty()
                && (Instant::now()
                    .duration_since(v.get_time_of_empty())
                    .as_secs()
                    > self.inactive_bucket_timeout)
        });
    }

    fn get_bucket_queue(&mut self, route: &RequestRoute) -> Option<&mut BucketQueue> {
        self.queue_map.get_mut(route)
    }

    fn notify_empty(&mut self, route: &RequestRoute) {
        self.active_requests_set.remove(route);
        self.queue_map.get_mut(route).unwrap().time_of_empty = Instant::now()
    }

    fn is_empty(&self) -> bool {
        !self.queue_map.iter().any(|v| !v.1.is_empty())
    }
}
