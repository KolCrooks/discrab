use crossbeam_channel::Sender;

use crate::core::{http::rate_limit_client::RequestObject, settings::Settings};

#[derive(Clone)]
pub struct Context {
    pub token: String,
    pub request_stream: Sender<RequestObject>,
    pub settings: Settings,
    pub cache: (), // TODO
}
