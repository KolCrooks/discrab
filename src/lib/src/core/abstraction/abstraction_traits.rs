use std::fmt::{Debug, Display};

use async_trait::async_trait;
use futures_util::Future;

use crate::{
    core::interactions::{interaction_event::InteractionCreate, typing::Interaction},
    discord::interactions::application_command::ApplicationCommandOption,
    ApplicationCommandType, Context, Events, Snowflake,
};

use super::{event_dispatcher::EventDispatcher, interaction_router::InteractionRouter};

/// Used to allow structs to be registered as event handlers for either interactions or general events.
///
/// Users shouldn't need to implement this trait, and can use the `#[event_handler]` or `#[command]` attribute instead.
/// Both of these macros will also implement EventHandlerImpl.
///
/// ### Events
/// The most basic way of registering an event is doing dispatcher.(EVENT_TYPE).register(self). dispatcher observers just accept objects that implement EventHandlerImpl
///
/// ### Interactions
/// dispatcher.register_interaction(id, self)
///
/// ### Examples:
/// #### Event Handler
/// ```rust,no_run
/// struct MyStruct;
/// impl EventHandlerImpl<InteractionCreate> for MyStruct { ... }
///
/// impl<'a> discord_rs::Registerable<'a> for MyStruct {
///     fn register(
///         &'a self,
///         ctx: discord_rs::Context,
///         _: &mut discord_rs::EventDispatcher<'a>,
///         interaction_router: &mut discord_rs::InteractionRouter<'a>,
///     ){
///         // Get the id of the interaction handler, or create a new one if it doesn't exist
///         let id = async_std::task::block_on(discord_rs::InteractionRouter::get_id_or_register::<MyStruct>(ctx));
///         // Register the handler
///         interaction_router.register_command(id, self);
///     }
/// }
/// ```
///
/// #### Interaction
/// ```rust,no_run
/// struct MyStruct;
/// impl EventHandlerImpl<InteractionCreate> for MyStruct { ... }
///
/// impl<'a> discord_rs::Registerable<'a> for MyStruct {
///     fn register(
///         &'a self,
///         ctx: discord_rs::Context,
///         dispatcher: &mut discord_rs::EventDispatcher<'a>,
///         _: &mut discord_rs::InteractionRouter<'a>,
///     ) {
///         dispatcher.get_observable(MyStruct::EVENT_TYPE, "InteractionCreate").subscribe(self);
///     }
/// }
/// ```
///
pub trait Registerable<'a> {
    fn register(
        &'a self,
        ctx: Context,
        dispatcher: &mut EventDispatcher<'a>,
        interaction_router: &mut InteractionRouter<'a>,
    );
}

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
pub trait InternalEventHandler<T: CommandArg> {
    /// This function is called by the event dispatcher or interaction handler.
    fn handler(&self, _: Context, _: T);
}

#[async_trait]
pub trait CommandHandler {
    /// The type of the command.
    ///
    /// **ChatInput**: Slash commands; a text-based command that shows up when a user types `/`
    ///
    /// **User**: A UI-based command that shows up when you right click or tap on a user
    ///
    /// **Message**: A UI-based command that shows up when you right click or tap on a message
    const COMMAND_TYPE: ApplicationCommandType;
    /// The name of the command.
    const COMMAND_NAME: &'static str;
    /// The description of the command.
    const COMMAND_DESCRIPTION: &'static str;
    /// The guild ID that the command is restricted to
    const GUILD_ID: Option<Snowflake> = None;

    /// This will be called when the command is registered so that the user can define the options for the command.
    /// TODO: Add documentation about creating options
    fn get_options() -> Vec<ApplicationCommandOption> {
        Vec::new()
    }

    /// This function is called when the interaction associated with the command is triggered.
    async fn handler(&self, _: Context, _: InteractionCreate);
}

/// Makes the user only able to use structs that implement CommandArg in their EventHandler
pub trait CommandArg {}
