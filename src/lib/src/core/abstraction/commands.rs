use async_trait::async_trait;

use crate::{core::interactions::typing::Interaction, Context, Events};

use super::event_dispatcher::EventDispatcher;

pub trait Registerable {
    fn register(&self, dispatcher: &mut EventDispatcher);
}

pub trait RegisterableImpl<T: CommandArg>: EventHandler<T> + Registerable {
    const EVENT_TYPE: Events;
}

#[async_trait]
pub trait EventHandler<T: CommandArg> {
    async fn handler(_: Context, _: T);
}

#[async_trait]
pub trait ApplicationCommandHandler: EventHandler<Interaction> {
    async fn handler(_: Context, _: Interaction);
}

pub trait CommandArg {}
