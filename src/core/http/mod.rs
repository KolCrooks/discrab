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

pub struct RateLimitedHttpClient {
    send_queue: Arc<Mutex<request_queue::Queue>>,
}

impl Default for RateLimitedHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitedHttpClient {
    pub fn new() -> RateLimitedHttpClient {
        RateLimitedHttpClient {
            send_queue: Arc::new(Mutex::new(request_queue::Queue::new())),
        }
    }

    /**
     * Spawn the request loop
     */
    // TODO maybe make this be called automatically when the client is created?
    pub fn spawn_req_thread(&mut self) {
        let send_queue = self.send_queue.clone();
        unsafe impl Send for request_queue::Queue {}

        request_thread::create_thread(send_queue);
    }

    /**
     * Send a request. This will queue the request and then execute when it is able to.
     *
     * @param route The route identifier that the request belongs to
     * @param request The request to send
     * @return The response from discord
     */
    pub async fn send_request(
        &self,
        route: RequestRoute,
        request: Request<Body>,
    ) -> Result<hyper::Response<Body>, Error> {
        let mut future = request_future::HttpFuture::new(request);
        // TODO Maybe use req_thread.unpark() to reduce cpu load while the thread is waiting for requests.
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
