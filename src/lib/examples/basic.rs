use discord_rs::{
    command,
    command_args::InteractionCreate,
    core::interactions::typing::InteractionCallbackData,
    event_handler,
    resources::{Channel, Message},
    ApplicationCommandType, Bot, CommandHandler, Context, EventHandler, Events,
};

use std::env;

struct MsgEvent;

#[event_handler]
impl EventHandler<Message> for MsgEvent {
    /// This is going to be the event that the handler will listen for
    const EVENT_TYPE: Events = Events::MessageCreate;

    /// This function is called when the bot receives event with Self::EVENT_TYPE
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

#[command]
impl CommandHandler for AppCmd {
    const COMMAND_TYPE: ApplicationCommandType = ApplicationCommandType::ChatInput;
    const COMMAND_NAME: &'static str = "ping";
    const COMMAND_DESCRIPTION: &'static str = "pong!";

    async fn handler(&self, _: Context, interaction: InteractionCreate) {
        interaction
            .respond_message(InteractionCallbackData::message_from_str(
                "pong!".to_string(),
            ))
            .await
            .unwrap();
    }

    const GUILD_ID: Option<discord_rs::Snowflake> = None;
}

#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();
    let token = env::var("TOKEN").unwrap();

    let mut bot = Bot::new(token);
    bot.settings().set_debug(true);
    bot.register_all(vec![&MsgEvent]).listen().await;
}
