use discrab::CommandHandler;
use discrab::api::{ApplicationCommandType, ApplicationCommandOption};
use discrab::core::interactions::typing::InteractionCallbackData;
use discrab::events::InteractionCtx;
use discrab::builders::OptionBuilder;
use discrab::macros::*;


pub struct EchoCmd;

#[command]
impl CommandHandler for EchoCmd {
    const COMMAND_TYPE: ApplicationCommandType = ApplicationCommandType::ChatInput;
    const NAME: &'static str = "echo";
    const DESCRIPTION: &'static str = "ECHO THE MESSAGE";

    async fn handler(&self, interaction: InteractionCtx) {
        let option = interaction.get_option::<String>("message").unwrap();
        
        interaction
            .respond_message(InteractionCallbackData::message_from_str(
                option.value.to_string()
            ))
            .await
            .unwrap();
    }

    fn get_options() ->  Vec<ApplicationCommandOption> {
        vec![
            OptionBuilder::new_str("message".to_string(), "message to echo".to_string())
            .required(true)
            .build()
        ]
    }
}