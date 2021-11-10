use std::sync::{Arc, Mutex};

use hyper::{Body, Error, Request};

mod request_bucket;
mod request_future;
mod request_queue;
mod request_thread;

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

        request_thread::create_thread(send_queue);
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
