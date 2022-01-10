use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use hyper::{Body, Error, Request};

/// This future is used by the http client to transfer data about a request between threads.
/// The future will initially be send to the http client, and then the http client will make the request, send the request response to the future,
/// and then wake the future up. This will unblock the request method, and then the future will unblock.
pub struct HttpFuture {
    /// State of the request
    pub shared_state: Arc<Mutex<RequestState>>,
}

/// State of the request to be shared between threads
pub struct RequestState {
    /// The request
    pub request: Option<Request<Body>>,

    /// The response that gets created once it is done
    pub response: Option<Result<hyper::Response<Body>, Error>>,

    /// Signals that the request has finished
    pub waker: Option<Waker>,
}

impl RequestState {
    /// Commits data to the request state, and then wakes up the task so that the async block can unblock
    pub fn commit(&mut self, response: Result<hyper::Response<Body>, Error>) {
        self.response = Some(response);
        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref()
        }
    }
}

impl HttpFuture {
    /// Creates a new future with the given request
    pub fn new(request: Request<Body>) -> Self {
        let shared_state = Arc::new(Mutex::new(RequestState {
            request: Some(request),
            response: None,
            waker: None,
        }));

        HttpFuture { shared_state }
    }
}

impl Future for HttpFuture {
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
