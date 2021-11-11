use std::sync::{Arc, Mutex};

use hyper::{Body, Error, Request};

use super::{
    request_future,
    request_queue::{self, BasicHttpQueue, HttpQueue},
    request_thread,
};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct RequestRoute {
    pub base_route: String,
    pub major_param: String,
}

pub struct RateLimitedHttpClient<T>
where
    T: HttpQueue + Send + 'static,
{
    send_queue: Arc<Mutex<T>>,
}

// impl<BasicHttpQueue> Default for RateLimitedHttpClient<BasicHttpQueue>
// where
//     T: HttpQueue + Send + 'static,
// {
//     fn default() -> Self {
//         Self::new(BasicHttpQueue::new(2));
//     }
// }

impl<T> RateLimitedHttpClient<T>
where
    T: HttpQueue + Send + 'static,
{
    pub fn new(queue: T) -> RateLimitedHttpClient<T> {
        RateLimitedHttpClient {
            send_queue: Arc::new(Mutex::new(queue)),
        }
    }

    /**
     * Spawn the request loop
     */
    // TODO maybe make this be called automatically when the client is created?
    pub fn spawn_req_thread(&mut self) {
        let send_queue = self.send_queue.clone();

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
