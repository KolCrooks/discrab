use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use hyper::{Body, Error, Request};

pub struct ReqFuture {
    /// State of the request
    pub shared_state: Arc<Mutex<RequestState>>,
}

pub struct RequestState {
    /// The request
    pub request: Option<Request<Body>>,

    /// The response that gets created once it is done
    pub response: Option<Result<hyper::Response<Body>, Error>>,

    /// Signals that the request has finished
    pub waker: Option<Waker>,
}

impl RequestState {
    pub fn commit(&mut self, response: Result<hyper::Response<Body>, Error>) {
        self.response = Some(response);
        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref()
        }
    }
}

impl ReqFuture {
    pub fn new(request: Request<Body>) -> Self {
        let shared_state = Arc::new(Mutex::new(RequestState {
            request: Some(request),
            response: None,
            waker: None,
        }));

        ReqFuture { shared_state }
    }
}

impl Future for ReqFuture {
    type Output = Result<hyper::Response<Body>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.response.is_some() {
            Poll::Ready(shared_state.response.take().unwrap())
        } else {
            // Look into `Waker::will_wake` so that we don't have to clone the waker again and again
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
