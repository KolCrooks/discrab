use std::panic::{UnwindSafe, RefUnwindSafe};

use async_trait::async_trait;

use crate::{Context, Events};


/// Makes the user only able to use structs that implement CommandArg in their EventHandler
pub trait CommandArg {}

/// This trait is used to help users create event handlers for the event dispatcher.
/// When combined with the `#[event_handler]` macro, this struct will be used to implement the `InternalEventHandler` and the `Registerable` traits.
/// The reason why this struct is needed is that handlers can't have consts, and also it allows the user to have an async function as the handler.
#[async_trait]
pub trait EventHandler<T: CommandArg> {
    const EVENT_TYPE: Events;

    async fn handler(&self, _: Context, _: T);
}

/// This trait is used behind the scenes to wrap the user's event handler,
/// and enable it to be called by the event dispatcher or interaction handler as a sync function.
/// It is implemented by the `#[event_handler]` or the `#[command]` macro.
pub trait __InternalEventHandler<T: CommandArg>: UnwindSafe + RefUnwindSafe {
    /// This function is called by the event dispatcher or interaction handler.
    fn handler(&self, _: Context, _: T);
}