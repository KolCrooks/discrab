use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{
    api::ApplicationCommand,
    api::{Snowflake, ApplicationCommandOption},
    core::interactions::{interaction_event::InteractionCtx, typing::Interaction},
    discord::interactions::application_command::{CreateApplicationCommand, EditApplicationCommand},
    util::{logger::print_debug, common::options_equal},
    CommandHandler, Context, Registerable,
};

use super::traits::__InternalEventHandler;

/// This is used to dispatch interaction events to the correct handler
pub struct InteractionRouter {
    pub commands: Mutex<HashMap<Snowflake, Arc<dyn __InternalEventHandler<InteractionCtx>>>>,
}

impl __InternalEventHandler<Interaction> for InteractionRouter {
    /// Handles the incomming interaction from the event dispatcher, and then forawrds it to the correct handler
    fn handler(&self, ctx: Context, interaction: Interaction) {
        // Get the id of the interaction
        let id = interaction
            .data
            .as_ref()
            .expect("Interaction doesn't have ID!")
            .id;
        // Get the handler and then call it
        let _commands = self.commands.lock().unwrap();
        let command = _commands.get(&id);
        if let Some(command) = command {
            command.handler(
                ctx.clone(),
                InteractionCtx::from_interaction(ctx, interaction),
            );
        } else if ctx.settings.debug {
            print_debug(
                "INTERACTIONS",
                format!("Unable to route interaction {}, interactions: {:?}", id, _commands.keys()),
            );
        }
    }
}

impl InteractionRouter {
    /// Creates a new interaction router
    pub fn new() -> Self {
        Self {
            commands: Mutex::new(HashMap::new()),
        }
    }

    /// Registers a new interaction handler
    pub fn register_command(
        &self,
        id: Snowflake,
        cmd: Arc<dyn __InternalEventHandler<InteractionCtx>>,
    ) {
        self.commands.lock().unwrap().insert(id, cmd);
    }

    /// Gets the id of the interaction handler if it exists. If it doesn't exist, it registers a new one and returns the id
    pub async fn get_id_or_register<T: CommandHandler + Registerable>(ctx: Context, handler: Arc<T>) -> Snowflake {
        if ctx.settings.debug {
            print_debug(
                "INTERACTIONS",
                format!("Registering command: {}", T::NAME),
            );
        }

        // Get the information from the command handler
        let mut options_raw = handler.get_options();
        let options = 
        if options_raw.is_empty() && handler.get_subs().is_none() {
            None
        } else if !options_raw.is_empty() {
            if handler.get_subs().is_some() {
                panic!("Command [{}] can't have both options and subs!", T::NAME);
            }
            Some(options_raw)
        }
        else {
            if !options_raw.is_empty() {
                panic!("Command [{}] can't have both options and subs!", T::NAME);
            }
            for sub in handler.get_subs().unwrap() {
                let sub_options = sub.get_options();
                options_raw.push(ApplicationCommandOption {
                    name: sub.get_name().unwrap().to_string(),
                    description: sub.get_description().unwrap().to_string(),
                    type_: sub.get_reg_type().into(),
                    options: if sub_options.is_empty() {
                        None
                    } else {
                        Some(sub_options)
                    },
                    ..Default::default()
                });
            }
            Some(options_raw)
        };
        if ctx.settings.debug {
            print_debug("INTERACTIONS",format!("{} Options: {:?}", T::NAME, options));
        }

        // Get all the commands
        match ApplicationCommand::list_global(ctx.clone())
            .await
            .unwrap()
            .into_iter()
            // Find the command that has the same name, type, and guild_id as the command handler
            .find(|cmd| {
                cmd.type_ == T::COMMAND_TYPE
                    && cmd.name == T::NAME
                    && cmd.guild_id == T::GUILD_ID
            }) {
            // The command exists, so return the id
            Some(cmd) => {
                if Some(T::DESCRIPTION.to_string()) != cmd.description ||
                    !options_equal(&options, &cmd.options) ||
                    cmd.default_permission != true ||
                    cmd.default_member_permissions != None ||
                    cmd.dm_permission != None {
                    // The command exists, but it doesn't match the command handler, so we need to edit it
                    if ctx.settings.debug {
                        print_debug(
                            "INTERACTIONS",
                            format!(
                                "Command Out of date. Updating: {}",
                                T::NAME
                            ),
                        );
                    }
                    ApplicationCommand::edit_global(
                        ctx,
                        cmd.id,
                        EditApplicationCommand {
                            name: Some(T::NAME.to_string()),
                            description: Some(T::DESCRIPTION.to_string()),
                            options,
                            default_permission: Some(true), // TODO make this user changeable
                            default_member_permissions: None, // TODO replace default_permission with this
                            dm_permission: None, // TODO make this user changeable
                        },
                    )
                    .await
                    .unwrap();
                    cmd.id
                    }
                else {
                    cmd.id
                }
            },
            // The command doesn't exist, so register it and return the id
            None => {
                if ctx.settings.debug {
                    print_debug(
                        "INTERACTIONS",
                        format!(
                            "[{}] Command not found so Creating new one",
                            T::NAME
                        ),
                    );
                }
                let cmd = ApplicationCommand::create_global(
                    ctx,
                    CreateApplicationCommand {
                        name: T::NAME.to_string(),
                        description: T::DESCRIPTION.to_string(),
                        options,
                        default_permission: Some(true), // TODO make this user changeable
                        default_member_permissions: None, // TODO replace default_permission with this
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

impl Default for InteractionRouter {
    fn default() -> Self {
        Self::new()
    }
}
