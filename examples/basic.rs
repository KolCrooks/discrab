use discord_rs::core::{
    http::{rate_limit_client::RLClient, request_queue::BasicHttpQueue},
    interactions::{
        self,
        handler::{websocket::WebsocketInteractionHandler, InteractionHandler},
    },
};
use serde_json::json;

// use dotenv::dotenv;
use std::{env, thread, time::Duration};
#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();
    let token = env::var("TOKEN").unwrap();
    let client = RLClient::new(token.clone(), BasicHttpQueue::new(60));
    let interactionHandler = WebsocketInteractionHandler::create(&client).await;

    let cmd = json!({
        "op": 2,
        "d": {
            "token": token,
            "properties": {
                "$os": "linux",
                "$browser": "discord.rs",
                "$device": "discord.rs",
            },
            "intents": 1 << 9,
        }
    });
    interactionHandler.send_command(cmd.to_string());

    loop {
        thread::sleep(Duration::from_millis(1));
    }
}
