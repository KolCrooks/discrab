use async_trait::async_trait;

use discord_rs::{
    application_command,
    command_args::Interaction,
    event_handler,
    resources::{Channel, Message},
    ApplicationCommand, ApplicationCommandHandler, ApplicationCommandType, Context, EventHandler,
    Events,
};

use std::env;

struct MsgEvent;

#[async_trait]
#[event_handler]
impl EventHandler<Message> for MsgEvent {
    const EVENT_TYPE: Events = Events::MessageCreate;
    async fn handler(ctx: Context, msg: Message) {
        if msg.content.starts_with("!ping") {
            Channel::send_message(ctx.clone(), msg.channel_id.to_string(), "pong".to_string())
                .await
                .unwrap();
        }
    }
}

struct AppCmd;

#[async_trait]
#[application_command]
impl ApplicationCommandHandler for AppCmd {
    const COMMAND_TYPE: ApplicationCommandType = ApplicationCommandType::Message;
    async fn handler(ctx: Context, msg: Interaction) {
        if msg.data.unwrap().starts_with("!ping") {
            Channel::send_message(
                ctx.clone(),
                msg.channel_id.unwrap().to_string(),
                "pong".to_string(),
            )
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
