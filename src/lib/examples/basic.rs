use async_trait::async_trait;

use discord_rs::{
    command,
    command_args::Interaction,
    event_handler,
    resources::{Channel, Message},
    ApplicationCommandType, Bot, CommandHandler, Context, EventHandler, Events,
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
#[command]
impl CommandHandler for AppCmd {
    const COMMAND_TYPE: ApplicationCommandType = ApplicationCommandType::ChatInput;
    const COMMAND_NAME: &'static str = "ping";
    const COMMAND_DESCRIPTION: &'static str = "pong!";

    async fn handler(ctx: Context, interaction: Interaction) {
        Channel::send_message(
            ctx.clone(),
            interaction.channel_id.unwrap().to_string(),
            "pong!".to_string(),
        )
        .await
        .unwrap();
    }
}

#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();
    let token = env::var("TOKEN").unwrap();

    Bot::new(token)
        .register_all(vec![&MsgEvent, &AppCmd])
        .listen()
        .await;
}
