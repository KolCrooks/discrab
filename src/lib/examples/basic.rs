use async_trait::async_trait;

use discord_rs::{
    resources::{Channel, Message},
    Context, EventHandler, Events,
};
use discordrs_codegen::event_handler;

// use dotenv::dotenv;
use std::env;

struct MsgEvent;

#[async_trait]
#[event_handler(Events::message_create)]
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
