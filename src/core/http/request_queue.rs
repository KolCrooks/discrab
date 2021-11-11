use std::{
    collections::{HashMap, HashSet, LinkedList, VecDeque},
    mem::size_of_val,
    sync::Mutex,
    time::Instant,
};

use super::{rate_limit_client::RequestRoute, request_future};

pub struct BucketQueue {
    time_of_empty: Instant,
    queue: LinkedList<(u64, *mut request_future::HttpFuture)>,
}

impl BucketQueue {
    pub fn new() -> BucketQueue {
        BucketQueue {
            time_of_empty: Instant::now(),
            queue: LinkedList::new(),
        }
    }

    pub fn push(&mut self, time: u64, future: *mut request_future::HttpFuture) {
        self.queue.push_back((time, future));
    }

    pub fn get_oldest(&self) -> Option<&(u64, *mut request_future::HttpFuture)> {
        self.queue.front()
    }

    pub fn pop(&mut self) -> Option<(u64, *mut request_future::HttpFuture)> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
impl Default for BucketQueue {
    fn default() -> Self {
        BucketQueue::new()
    }
}

pub struct BasicHttpQueue {
    memory_limit: usize,
    empty_buckets: u32,
    req_id_cnt: u64,
    queue_map: HashMap<RequestRoute, BucketQueue>,
    active_requests_set: HashSet<RequestRoute>,
}

impl BasicHttpQueue {
    fn new(memory_limit: usize) -> BasicHttpQueue {
        BasicHttpQueue {
            req_id_cnt: 0,
            memory_limit,
            empty_buckets: 0,
            queue_map: HashMap::new(),
            active_requests_set: HashSet::new(),
        }
    }
}

pub trait HttpQueue {
    fn push(&mut self, route: &RequestRoute, future: *mut request_future::HttpFuture);
    fn get_sorted_requests(&self) -> Vec<RequestRoute>;
    fn get_bucket_queue(&mut self, route: &RequestRoute) -> Option<&mut BucketQueue>;
    fn notify_empty(&mut self, route: &RequestRoute);
    fn clean_up(&mut self, route: &RequestRoute);
}

impl HttpQueue for BasicHttpQueue {
    fn push(&mut self, route: &RequestRoute, future: *mut request_future::HttpFuture) {
        let queue = self.queue_map.entry(route.clone()).or_insert_with(|| {
            self.empty_buckets += 1;
            BucketQueue::new()
        });

        if queue.is_empty() {
            self.empty_buckets -= 1;
        }

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

    /**
     * Scheduled cleanup that runs every 10 sec, or when
     */
    fn clean_up(&mut self, route: &RequestRoute) {
        let mut total_usage = 0;
    }

    fn get_bucket_queue(&mut self, route: &RequestRoute) -> Option<&mut BucketQueue> {
        self.queue_map.get_mut(route)
    }

    fn notify_empty(&mut self, route: &RequestRoute) {
        self.active_requests_set.remove(&route);
        self.empty_buckets += 1;
        self.queue_map.get_mut(route).unwrap().time_of_empty = Instant::now()
    }
}
