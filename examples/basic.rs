use discord_rs::{
    core::{
        http::{rate_limit_client::RLClient, request_queue::BasicHttpQueue},
        interactions::{
            self,
            handler::{websocket::WebsocketEventHandler, SocketClient},
        },
    },
    discord::resources::channel::message::Message,
    Context,
};
use serde_json::json;

// use dotenv::dotenv;
use std::{env, thread, time::Duration};

fn test_msg_send(ctx: Context, msg: Message) {
    println!("{}", msg.content);
}

#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();
    let token = env::var("TOKEN").unwrap();

    let mut builder = discord_rs::BotBuilder::new(token);
    builder
        .event_dispatcher
        .message_create
        .subscribe(&test_msg_send);

    let bot = builder.build().await;

    loop {
        thread::sleep(Duration::from_millis(1));
    }
}
