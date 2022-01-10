use crossbeam_channel::Sender;

use crate::core::{http::rate_limit_client::RequestObject, settings::Settings};

/// Context object that is passed to all parts of the bot
/// It contains key information so that methods can create requests to discord, and also contains settings for those functions
#[derive(Clone)]
pub struct Context {
    /// The token for the bot's instance
    pub token: String,
    /// The request sender for the instance's bot. Allows the user to make http requests
    pub request_stream: Sender<RequestObject>,
    /// The settings for the bot's instance
    pub settings: Settings,
    /// The cache for the bot's instance
    pub cache: (), // TODO
}
