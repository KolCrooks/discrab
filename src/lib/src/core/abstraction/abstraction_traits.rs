use async_trait::async_trait;

use crate::{
    core::interactions::typing::Interaction,
    discord::interactions::application_command::ApplicationCommandOption, ApplicationCommandType,
    Context, Events, Snowflake,
};

use super::{event_dispatcher::EventDispatcher, interaction_router::InteractionRouter};

pub trait Registerable {
    fn register(
        &self,
        ctx: Context,
        dispatcher: &mut EventDispatcher,
        interaction_router: &mut InteractionRouter,
    );
}

#[async_trait]
pub trait EventHandler<T: CommandArg> {
    const EVENT_TYPE: Events;

    async fn handler(_: Context, _: T);
}

#[async_trait]
pub trait CommandHandlerImpl {
    async fn handler(_: Context, _: Interaction);
}

#[async_trait]
pub trait CommandHandler {
    const COMMAND_TYPE: ApplicationCommandType;
    const COMMAND_NAME: &'static str;
    const COMMAND_DESCRIPTION: &'static str;
    const GUILD_ID: Option<Snowflake> = None;

    async fn handler(_: Context, _: Interaction);

    fn get_options() -> Vec<ApplicationCommandOption> {
        Vec::new()
    }
}

pub trait CommandArg {}
