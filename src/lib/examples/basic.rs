use async_trait::async_trait;

use async_std::task::block_on;
use discord_rs::core::abstraction::commands::EventHandler;
use discord_rs::core::abstraction::event_dispatcher::EventDispatcher;
use discord_rs::discord::resources::channel::Channel;
use discord_rs::{discord::resources::channel::message::Message, Context, Events};
use discordrs_codegen::event_handler;
use serde_json::json;

// use dotenv::dotenv;
use std::{env, thread, time::Duration};

struct MsgEvent;

#[async_trait]
#[event_handler("MESSAGE_CREATE")]
impl EventHandler<Message> for MsgEvent {
    async fn handler(ctx: Context, msg: Message) {
        if msg.content.starts_with("!ping") {
            Channel::send_message(ctx.clone(), msg.channel_id.to_string(), "pong".to_string())
                .await
                .unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();
    let token = env::var("TOKEN").unwrap();

    let mut builder = discord_rs::BotBuilder::new(token);
    builder.register_all(vec![&MsgEvent]);
    let bot = builder.build();
    bot.listen().await;
}
