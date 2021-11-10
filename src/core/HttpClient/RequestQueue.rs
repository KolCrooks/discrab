use std::{
    collections::{HashMap, HashSet, LinkedList, VecDeque},
    sync::Mutex,
};

use super::{RequestFuture, RequestRoute};

pub struct Queue {
    pub queue_map: HashMap<RequestRoute, Mutex<LinkedList<*const RequestFuture::ReqFuture>>>,
    pub active_requests_set: HashSet<RequestRoute>,
    pub active_requests_queue: LinkedList<RequestRoute>,
}

struct ItemContext {}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            queue_map: HashMap::new(),
            active_requests_set: HashSet::new(),
            active_requests_queue: LinkedList::new(),
        }
    }

    pub fn push(&mut self, route: RequestRoute, future: *const RequestFuture::ReqFuture) {
        let mut queue = self
            .queue_map
            .entry(route)
            .or_insert(Mutex::new(LinkedList::new()));

        queue.lock().unwrap().push_back(future);
        if !self.active_requests_set.contains(&route) {
            self.active_requests_set.insert(route);
            self.active_requests_queue.push_back(route);
        }
    }
}
