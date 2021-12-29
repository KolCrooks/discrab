use async_trait::async_trait;

use crate::Context;

use super::event_dispatcher::EventDispatcher;

pub trait Registerable {
    fn register(&self, dispatcher: &mut EventDispatcher);
}

#[async_trait]
pub trait EventHandler<T: CommandArg> {
    async fn handler(_: Context, _: T);
}

pub trait CommandArg {}
