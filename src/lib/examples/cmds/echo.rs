use discrab::CommandHandler;
use discrab::api::{ApplicationCommandType, ApplicationCommandOption};
use discrab::core::interactions::typing::InteractionCallbackData;
use discrab::events::InteractionCtx;
use discrab::builders::OptionBuilder;
use discrab::macros::*;


pub struct EchoCmd;

#[command]
impl CommandHandler<'_> for EchoCmd {
    const COMMAND_TYPE: ApplicationCommandType = ApplicationCommandType::ChatInput;
    const COMMAND_NAME: &'static str = "echo";
    const COMMAND_DESCRIPTION: &'static str = "ECHO THE MESSAGE";

    async fn handler(&self, interaction: InteractionCtx) {
        let option = interaction.data.as_ref().unwrap().options.as_ref().unwrap().iter().find(|o|o.name == "message").unwrap();
        let message = option.value.as_ref().unwrap().as_str();
        interaction
            .respond_message(InteractionCallbackData::message_from_str(
                message.to_string()
            ))
            .await
            .unwrap();
    }
    fn get_options() ->  Vec<ApplicationCommandOption> {
        vec![
            OptionBuilder::new_str("message".to_string())
            .required(true)
            .build()
        ]
    }
}