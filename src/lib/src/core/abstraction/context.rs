use crossbeam_channel::Sender;

use crate::core::http::rate_limit_client::RequestObject;

#[derive(Clone)]
pub struct Context {
    pub token: String,
    pub request_stream: Sender<RequestObject>,
    pub cache: (), // TODO
}
