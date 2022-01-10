use std::fmt::{Debug, Display};

use async_trait::async_trait;
use futures_util::Future;

use crate::{
    core::interactions::{interaction_event::InteractionCreate, typing::Interaction},
    discord::interactions::application_command::ApplicationCommandOption,
    ApplicationCommandType, Context, Events, Snowflake,
};

use super::{event_dispatcher::EventDispatcher, interaction_router::InteractionRouter};

pub trait Registerable<'a> {
    fn register(
        &'a self,
        ctx: Context,
        dispatcher: &mut EventDispatcher<'a>,
        interaction_router: &mut InteractionRouter<'a>,
    );
}

#[async_trait]
pub trait EventHandler<T: CommandArg> {
    const EVENT_TYPE: Events;

    async fn handler(&self, _: Context, _: T);
}

pub trait EventHandlerImpl<T: CommandArg> {
    fn handler(&self, _: Context, _: T);
}

impl<T: CommandArg> Debug for dyn EventHandlerImpl<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventHandlerImpl<{}>", std::any::type_name::<T>())
    }
}

#[async_trait]
pub trait CommandHandler {
    const COMMAND_TYPE: ApplicationCommandType;
    const COMMAND_NAME: &'static str;
    const COMMAND_DESCRIPTION: &'static str;
    const GUILD_ID: Option<Snowflake> = None;

    async fn handler(&self, _: Context, _: InteractionCreate);

    fn get_options() -> Vec<ApplicationCommandOption> {
        Vec::new()
    }
}

pub trait CommandArg {}
