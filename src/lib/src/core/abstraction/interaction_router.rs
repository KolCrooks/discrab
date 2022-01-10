use std::collections::HashMap;

use crate::{
    core::interactions::{interaction_event::InteractionCreate, typing::Interaction},
    discord::interactions::application_command::CreateApplicationCommand,
    util::logger::print_debug,
    ApplicationCommand, CommandHandler, Context, EventDispatcher, Snowflake,
};

use super::abstraction_traits::EventHandlerImpl;

pub struct InteractionRouter<'a> {
    pub commands: HashMap<Snowflake, &'a dyn EventHandlerImpl<InteractionCreate>>,
}

impl<'a> EventHandlerImpl<Interaction> for InteractionRouter<'a> {
    fn handler(&self, ctx: Context, interaction: Interaction) {
        let id = interaction
            .data
            .as_ref()
            .expect("Interaction doesn't have ID!")
            .id;
        let command = self.commands.get(&id);
        if let Some(command) = command {
            command.handler(
                ctx.clone(),
                InteractionCreate::from_interaction(ctx, interaction),
            );
        } else {
            print_debug(
                "INTERACTIONS",
                format!("Unable to route interaction {}", id),
            );
        }
    }
}

impl<'a> InteractionRouter<'a> {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register_command(
        &mut self,
        id: Snowflake,
        cmd: &'a dyn EventHandlerImpl<InteractionCreate>,
    ) {
        self.commands.insert(id, cmd);
    }

    pub async fn get_id_or_register<T: CommandHandler>(ctx: Context) -> Snowflake {
        print_debug(
            "INTERACTIONS",
            format!("Registering command: {}", T::COMMAND_NAME),
        );
        match ApplicationCommand::list_global(ctx.clone())
            .await
            .unwrap()
            .into_iter()
            .find(|cmd| {
                cmd.type_ == T::COMMAND_TYPE
                    && cmd.name == T::COMMAND_NAME
                    && cmd.guild_id == T::GUILD_ID
            }) {
            Some(cmd) => cmd.id,
            None => {
                print_debug(
                    "INTERACTIONS",
                    format!(
                        "[{}] Command not found so Creating new one",
                        T::COMMAND_NAME
                    ),
                );
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
