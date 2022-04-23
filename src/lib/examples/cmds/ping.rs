use discrab::CommandHandler;
use discrab::api::{ApplicationCommandType, ApplicationCommandOption};
use discrab::core::interactions::typing::InteractionCallbackData;
use discrab::events::InteractionCtx;
use discrab::macros::*;

pub struct PingSlashCmd {
    pub a: u32,
}

#[command]
impl CommandHandler<'_> for PingSlashCmd {
    const COMMAND_TYPE: ApplicationCommandType = ApplicationCommandType::ChatInput;
    const COMMAND_NAME: &'static str = "ping";
    const COMMAND_DESCRIPTION: &'static str = "pong!";

    async fn handler(&self, interaction: InteractionCtx) {
        interaction
            .respond_message(InteractionCallbackData::message_from_str(
                "pong!".to_string(),
            ))
            .await
            .unwrap();
    }
}