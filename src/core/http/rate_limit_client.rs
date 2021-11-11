use std::sync::{Arc, Mutex};

use hyper::{Body, Error, Request};

use super::{
    request_future::{self, HttpFuture},
    request_queue::{self, BasicHttpQueue, HttpQueue},
    request_thread,
};

use crossbeam_channel::{unbounded, Receiver, Sender};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct RequestRoute {
    pub base_route: String,
    pub major_param: String,
}

pub struct RequestObject {
    pub route: RequestRoute,
    pub future: *mut request_future::HttpFuture,
}

unsafe impl Send for RequestObject {}

impl RequestObject {
    pub fn new(route: RequestRoute, future: *mut request_future::HttpFuture) -> RequestObject {
        RequestObject { route, future }
    }
}

pub struct RateLimitedHttpClient {
    sender: Sender<RequestObject>,
}

impl Default for RateLimitedHttpClient {
    fn default() -> Self {
        Self::new(BasicHttpQueue::new(2))
    }
}

impl RateLimitedHttpClient {
    pub fn new<T>(queue: T) -> RateLimitedHttpClient
    where
        T: HttpQueue + Send + 'static,
    {
        let (s, r) = unbounded();
        let mut c = RateLimitedHttpClient { sender: s };
        c.spawn_req_thread::<T>(queue, r);
        c
    }

    /**
     * Spawn the request loop
     */
    // TODO maybe make this be called automatically when the client is created?
    pub fn spawn_req_thread<T>(&mut self, queue: T, receiver: Receiver<RequestObject>)
    where
        T: HttpQueue + Send + 'static,
    {
        request_thread::create_thread::<T>(queue, receiver);
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

        self.sender
            .send(RequestObject::new(route, &mut future as *mut _));

        future.await
    }
}
