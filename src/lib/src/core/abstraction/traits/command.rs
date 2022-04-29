use async_trait::async_trait;

use crate::{api::{ApplicationCommandType, Snowflake, ApplicationCommandOption, ApplicationCommandOptionType}, events::InteractionCtx, SubsVector, core::interactions::typing::InteractionData};


#[async_trait]
pub trait CommandHandler {
    /// The type of the command.
    ///
    /// **ChatInput**: Slash commands; a text-based command that shows up when a user types `/`
    ///
    /// **User**: A UI-based command that shows up when you right click or tap on a user
    ///
    /// **Message**: A UI-based command that shows up when you right click or tap on a message
    const COMMAND_TYPE: ApplicationCommandType = ApplicationCommandType::ChatInput;
    /// The name of the command.
    const NAME: &'static str;
    /// The description of the command.
    const DESCRIPTION: &'static str = "";
    /// The guild ID that the command is restricted to
    const GUILD_ID: Option<Snowflake> = None;

    /// This will be called when the command is registered so that the user can define the options for the command.
    /// TODO: Add documentation about creating options
    fn get_options() -> Vec<ApplicationCommandOption> {
        Vec::new()
    }

    /// This function is called when the interaction associated with the command is triggered.
    /// By default, this function will route the interaction down to any subcommands. If
    /// this function doesn't have any subcommands to route down to, it will panic.
    /// @param ctx The context of the interaction.
    async fn handler(&self, ctx: InteractionCtx) {
        if self.get_subs().is_none() {
            panic!("Command Handler for {} is not implimented!", Self::NAME);
        }
        self.route_down(ctx).await;
    }

    /// This function is called when the command is registered, and also every time a command
    /// is routed down to the appropriate subcommand.
    fn get_subs(&self) -> Option<&SubsVector> {
        None
    }


    async fn route_down(&self, ictx: InteractionCtx) {
        let sub: Vec<_> = ictx
        .data.as_ref().unwrap_or_else(|| panic!("Interaction [{}] has no data!", Self::NAME))
        .options.as_ref().unwrap_or_else(|| panic!("Interaction [{}] has no subroutes!", Self::NAME))
        .iter().filter(|opt| {
            opt.type_ == ApplicationCommandOptionType::SubCommandGroup ||
            opt.type_ == ApplicationCommandOptionType::SubCommand
        }).collect();
        
        if sub.is_empty() {
            panic!("Expected subroutes, but interaction [{}] did not reply with any!", Self::NAME);
        } else if sub.len() > 1 {
            panic!("Expected only one subroute, but interaction [{}] replied with more than one!", Self::NAME);
        } else {
            let s = sub.get(0).unwrap();
            let subs = self.get_subs().unwrap();
            let handler = 
                subs.iter()
                .find(|h|h.get_name().unwrap() == s.name.as_str())
                .unwrap_or_else(|| panic!("[{}] Sub-Route {} not found!", Self::NAME, s.name));
            
            let sub_ctx = InteractionCtx {
                data: Some(InteractionData {
                    options: s.options.clone(),
                    name: s.name.clone(),
                    ..ictx.data.clone().unwrap()
                }),
                ..ictx
            };
            handler.handler(sub_ctx).await;
        }
    }
}