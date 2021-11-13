use discord_rs::core::{
    http::{rate_limit_client::RLClient, request_queue::BasicHttpQueue},
    interactions,
};

// use dotenv::dotenv;
use std::{env, thread, time::Duration};
#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();

    let client = RLClient::new(env::var("TOKEN").unwrap(), BasicHttpQueue::new(60));
    let interactionHandler =
        interactions::handler::websocket::WebsocketInteractionHandler::new(&client);
    loop {
        thread::sleep(Duration::from_millis(1));
    }
}
