use async_trait::async_trait;
use discord_rs::{core::abstraction::event_dispatcher, EventHandler, Events};

#[macro_use]
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

pub struct msgEvent;

#[async_trait]
impl<T> EventHandler<T> for msgEvent {
    const EVENT: Events = Events::message_create;

    async fn handle(ctx: Context, msg: Message) {
        if msg.content.starts_with("!ping") {
            ctx.send_message(msg.channel_id, "pong").await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();
    let token = env::var("TOKEN").unwrap();

    let mut builder = discord_rs::BotBuilder::new(token);
    builder
        .event_dispatcher
        .get_observable(Events::message_create)
        .subscribe(&test_msg_send);

    let bot = builder.build();
    bot.listen().await;
}
