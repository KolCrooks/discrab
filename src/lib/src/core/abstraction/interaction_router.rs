use std::collections::HashMap;

use crate::{
    core::interactions::{interaction_event::InteractionCreate, typing::Interaction},
    discord::interactions::application_command::CreateApplicationCommand,
    util::logger::print_debug,
    ApplicationCommand, CommandHandler, Context, Snowflake,
};

use super::abstraction_traits::InternalEventHandler;

/// This is used to dispatch interaction events to the correct handler
pub struct InteractionRouter<'a> {
    pub commands: HashMap<Snowflake, &'a dyn InternalEventHandler<InteractionCreate>>,
}

impl<'a> InternalEventHandler<Interaction> for InteractionRouter<'a> {
    /// Handles the incomming interaction from the event dispatcher, and then forawrds it to the correct handler
    fn handler(&self, ctx: Context, interaction: Interaction) {
        // Get the id of the interaction
        let id = interaction
            .data
            .as_ref()
            .expect("Interaction doesn't have ID!")
            .id;
        /// Get the handler and then call it
        let command = self.commands.get(&id);
        if let Some(command) = command {
            command.handler(
                ctx.clone(),
                InteractionCreate::from_interaction(ctx, interaction),
            );
        } else if ctx.settings.debug {
            print_debug(
                "INTERACTIONS",
                format!("Unable to route interaction {}", id),
            );
        }
    }
}

impl<'a> InteractionRouter<'a> {
    /// Creates a new interaction router
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Registers a new interaction handler
    pub fn register_command(
        &mut self,
        id: Snowflake,
        cmd: &'a dyn InternalEventHandler<InteractionCreate>,
    ) {
        self.commands.insert(id, cmd);
    }

    /// Gets the id of the interaction handler if it exists. If it doesn't exist, it registers a new one and returns the id
    pub async fn get_id_or_register<T: CommandHandler>(ctx: Context) -> Snowflake {
        if ctx.settings.debug {
            print_debug(
                "INTERACTIONS",
                format!("Registering command: {}", T::COMMAND_NAME),
            );
        }

        // Get all the commands
        match ApplicationCommand::list_global(ctx.clone())
            .await
            .unwrap()
            .into_iter()
            // Find the command that has the same name, type, and guild_id as the command handler
            .find(|cmd| {
                cmd.type_ == T::COMMAND_TYPE
                    && cmd.name == T::COMMAND_NAME
                    && cmd.guild_id == T::GUILD_ID
            }) {
            // The command exists, so return the id
            Some(cmd) => cmd.id,
            // The command doesn't exist, so register it and return the id
            None => {
                if ctx.settings.debug {
                    print_debug(
                        "INTERACTIONS",
                        format!(
                            "[{}] Command not found so Creating new one",
                            T::COMMAND_NAME
                        ),
                    );
                }
                // Get the information from the command handler
                let options_raw = T::get_options();
                let options = if options_raw.is_empty() {
                    None
                } else {
                    Some(options_raw)
                };

                let cmd = ApplicationCommand::create_global(
                    ctx,
                    CreateApplicationCommand {
                        name: T::COMMAND_NAME.to_string(),
                        description: T::COMMAND_DESCRIPTION.to_string(),
                        options,
                        default_permission: Some(true), // TODO make this user changeable
                        type_: Some(T::COMMAND_TYPE),
                    },
                )
                .await
                .unwrap();
                cmd.id
            }
        }
    }
}

impl<'a> Default for InteractionRouter<'a> {
    fn default() -> Self {
        Self::new()
    }
}
