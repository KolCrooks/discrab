use async_trait::async_trait;

use crate::{core::interactions::typing::Interaction, ApplicationCommandType, Context, Events};

use super::event_dispatcher::EventDispatcher;

pub trait Registerable {
    fn register(&self, dispatcher: &mut EventDispatcher);
}

#[async_trait]
pub trait EventHandler<T: CommandArg> {
    const EVENT_TYPE: Events;

    async fn handler(_: Context, _: T);
}

#[async_trait]
pub trait ApplicationCommandHandler: EventHandler<Interaction> {
    const COMMAND_TYPE: ApplicationCommandType;

    async fn handler(_: Context, _: Interaction);
}

pub trait CommandArg {}
