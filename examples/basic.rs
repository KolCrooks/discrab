use discord_rs::core::{
    http::{
        rate_limit_client::{RLClient, RequestRoute},
        request_queue::BasicHttpQueue,
    },
    interactions,
};

use dotenv::dotenv;
use simd_json::Array;
use std::env;
#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();

    let client = RLClient::new(env::var("TOKEN").unwrap(), BasicHttpQueue::new(60));
    let interactionHandler =
        interactions::handler::websocket::WebsocketInteractionHandler::new(&client);
}
