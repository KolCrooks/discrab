use async_trait::async_trait;

use discord_rs::{
    command,
    command_args::{Interaction, InteractionCreate},
    core::interactions::typing::InteractionCallbackData,
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
    async fn handler(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!ping") {
            Channel::send_message(ctx.clone(), msg.channel_id.to_string(), "pong".to_string())
                .await
                .unwrap();
        }
    }
}

struct AppCmd {
    a: u32,
}

#[async_trait]
#[command]
impl CommandHandler for AppCmd {
    const COMMAND_TYPE: ApplicationCommandType = ApplicationCommandType::ChatInput;
    const COMMAND_NAME: &'static str = "ping";
    const COMMAND_DESCRIPTION: &'static str = "pong!";

    async fn handler(&self, ctx: Context, interaction: InteractionCreate) {
        // Channel::send_message(
        //     ctx.clone(),
        //     interaction.channel_id.unwrap().to_string(),
        //     "pong!".to_string(),
        // )
        // .await
        // .unwrap();
        interaction
            .respond_message(InteractionCallbackData::message_str("pong!".to_string()))
            .await
            .unwrap();
    }

    const GUILD_ID: Option<discord_rs::Snowflake> = None;
}

#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();
    let token = env::var("TOKEN").unwrap();

    Bot::new(token)
        .register_all(vec![&MsgEvent, &AppCmd { a: 1 }])
        .listen()
        .await;
}
