use hyper::{body::Body, header::AUTHORIZATION, Request};
use serde::de::DeserializeOwned;

use crate::{
    core::abstraction::context::Context,
    util::{error::Error, logger::print_debug},
};

use super::{
    request_future::{self},
    request_queue::HttpQueue,
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

pub struct RLClient {
    sender: Sender<RequestObject>,
}

impl RLClient {
    pub fn new<T>(queue: T) -> RLClient
    where
        T: HttpQueue + Send + 'static,
    {
        let (s, r) = unbounded();
        let mut c = RLClient { sender: s };
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

    pub fn get_req_sender(&self) -> Sender<RequestObject> {
        self.sender.clone()
    }
}

/**
 * Send a request. This will queue the request and then execute when it is able to.
 *
 * @param route The route identifier that the request belongs to
 * @param request The request to send
 * @return The response from discord
 */
pub async fn send_request<T: DeserializeOwned>(
    ctx: Context,
    route: RequestRoute,
    mut request: Request<Body>,
) -> Result<T, Error> {
    request
        .headers_mut()
        .insert(AUTHORIZATION, format!("Bot {}", ctx.token).parse().unwrap());

    let mut future = request_future::HttpFuture::new(request);
    // TODO Maybe use req_thread.unpark() to reduce cpu load while the thread is waiting for requests.
    // This would have the downside of increasing the power required make a request since we have to attempt to unpark it every time.
    // We could maybe get around this by having a parked flag, but this would require a mutex which also increases the power required.
    ctx.request_stream
        .send(RequestObject::new(route, &mut future as *mut _))
        .unwrap();

    let res = match future.await {
        Ok(res) => res,
        Err(e) => {
            print_debug("REQUEST", format!("Error: {:?}", e));
            return Err(Error::new(
                format!("{:?}", e),
                crate::util::error::ErrorTypes::REQUEST,
            ));
        }
    };
    let mut bytes = hyper::body::to_bytes(res).await.unwrap().to_vec();

    simd_json::from_slice::<T>(&mut *bytes).map_err(|e| {
        print_debug("REQUEST", format!("Error: {:?}", e));
        Error::new(format!("{:?}", e), crate::util::error::ErrorTypes::PARSE)
    })
}
