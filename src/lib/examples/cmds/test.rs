use std::sync::Arc;

use discrab::builders::OptionBuilder;
use discrab::core::interactions::typing::InteractionCallbackData;
use discrab::events::InteractionCtx;
use discrab::{CommandHandler, SubsVector};
use discrab::api::{ApplicationCommandType, ApplicationCommandOption};
use discrab::macros::*;

pub struct EchoSub;

#[subcommand]
impl CommandHandler for EchoSub {
    const NAME: &'static str = "echo2";
    const DESCRIPTION: &'static str = "ECHO THE MESSAGE";

    async fn handler(&self, interaction: InteractionCtx) {
        let option = interaction.get_option::<String>("message").unwrap();
        
        interaction
            .respond_message(discrab::core::interactions::typing::InteractionCallbackData::message_from_str(
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

pub struct PingSub;

#[subcommand]
impl CommandHandler for PingSub {
    const NAME: &'static str = "ping2";
    const DESCRIPTION: &'static str = "PONG!";

    async fn handler(&self, interaction: InteractionCtx) {
        interaction
            .respond_message(InteractionCallbackData::message_from_str(
                "pong!".to_string(),
            ))
            .await
            .unwrap();
    }
}

pub struct TestGroup {
    subs: SubsVector,
}

impl TestGroup {
    fn new() -> Self {
        Self {
            subs: vec![
                Arc::new(EchoSub),
                Arc::new(PingSub),
            ],
        }
    }
}


#[subcommand_group]
impl CommandHandler for TestGroup {
    const NAME: &'static str = "test";
    const DESCRIPTION: &'static str = "test group";
    
    fn get_subs(&self) -> Option<&SubsVector> {
        Some(&self.subs)
    }
}

pub struct TestCmd {
    subs: SubsVector,
}

impl TestCmd {
    pub fn new() -> Self {
        Self {
            subs: vec![
                Arc::new(TestGroup::new()),
            ],
        }
    }
}



#[command]
impl CommandHandler for TestCmd {
    const COMMAND_TYPE: ApplicationCommandType = ApplicationCommandType::ChatInput;
    const NAME: &'static str = "test";
    const DESCRIPTION: &'static str = "test command";
    
    fn get_subs(&self) -> Option<&SubsVector> {
        Some(&self.subs)
    }
}