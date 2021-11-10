use std::{
    collections::{HashMap, HashSet, LinkedList, VecDeque},
    sync::Mutex,
};

use super::{rate_limit_client::RequestRoute, request_future};

pub struct Queue {
    pub queue_map: HashMap<RequestRoute, Mutex<LinkedList<*mut request_future::HttpFuture>>>,
    pub active_requests_set: HashSet<RequestRoute>,
    pub active_requests_queue: VecDeque<RequestRoute>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            queue_map: HashMap::new(),
            active_requests_set: HashSet::new(),
            active_requests_queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, route: &RequestRoute, future: *mut request_future::HttpFuture) {
        let queue = self
            .queue_map
            .entry(route.clone())
            .or_insert_with(|| Mutex::new(LinkedList::new()));

        queue.get_mut().unwrap().push_back(future);
        if !self.active_requests_set.contains(route) {
            self.active_requests_set.insert(route.clone());
            self.active_requests_queue.push_back(route.clone());
        }
    }
}
