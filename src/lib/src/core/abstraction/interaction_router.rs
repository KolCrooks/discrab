use std::collections::HashMap;

use crate::{
    core::interactions::typing::Interaction,
    discord::interactions::application_command::CreateApplicationCommand,
    util::logger::print_debug, ApplicationCommand, ApplicationCommandType, CommandHandler, Context,
    EventDispatcher, Events, Registerable, Snowflake,
};

pub struct InteractionRouter {
    ctx: Context,
    pub commands: HashMap<Snowflake, Box<(dyn Fn(Context, Interaction) + Send + Sync)>>,
}

impl InteractionRouter {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
            commands: HashMap::new(),
        }
    }

    pub fn register_command(
        &mut self,
        id: Snowflake,
        command: Box<dyn Fn(Context, Interaction) + Send + Sync>,
    ) {
        self.commands.insert(id, command);
    }

    pub fn attatch(&mut self, event_dispatcher: &mut EventDispatcher) {
        event_dispatcher
            .get_observable_no_check(Events::InteractionCreate)
            .subscribe(&move |ctx, val| self.listener(ctx, val));
    }

    pub fn listener(&self, ctx: Context, val: Interaction) {
        let command = self.commands.get(&val.id);
        if let Some(command) = command {
            (**command)(ctx, val);
        } else {
            print_debug(
                "INTERACTIONS",
                format!("Unable to route interaction {}", val.id),
            );
        }
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
                        default_permission: None, // TODO make this user changeable
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
