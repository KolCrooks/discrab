use crate::{
    api::ApplicationCommandType, api::{Snowflake, ApplicationCommandOptionType},
    core::interactions::{interaction_event::InteractionCtx, typing::{InteractionData}},
    discord::interactions::application_command::ApplicationCommandOption, Context, Events,
};
use async_trait::async_trait;

use super::{event_dispatcher::EventDispatcher, interaction_router::InteractionRouter};

pub enum RegisterableType {
    Event,
    Command,
    SubCommandGroup,
    SubCommand
}

impl From<RegisterableType> for ApplicationCommandOptionType {
    fn from(rt: RegisterableType) -> Self {
       match rt {
           RegisterableType::SubCommandGroup => Self::SubCommandGroup,
           RegisterableType::SubCommand => Self::SubCommand,
           _ => panic!("This enum value is not convertable!")
       }
    }
}

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
/// impl EventHandlerImpl<InteractionCtx> for MyStruct { ... }
///
/// impl<'a> discrab::Registerable<'a> for MyStruct {
///     fn register(
///         &'a self,
///         ctx: discrab::Context,
///         _: &mut discrab::EventDispatcher<'a>,
///         interaction_router: &mut discrab::InteractionRouter<'a>,
///     ){
///         // Get the id of the interaction handler, or create a new one if it doesn't exist
///         let id = async_std::task::block_on(discrab::InteractionRouter::get_id_or_register::<MyStruct>(ctx));
///         // Register the handler
///         interaction_router.register_command(id, self);
///     }
/// }
/// ```
///
/// #### Interaction
/// ```rust,no_run
/// struct MyStruct;
/// impl EventHandlerImpl<InteractionCtx> for MyStruct { ... }
///
/// impl<'a> discrab::Registerable<'a> for MyStruct {
///     fn register(
///         &'a self,
///         ctx: discrab::Context,
///         dispatcher: &mut discrab::EventDispatcher<'a>,
///         _: &mut discrab::InteractionRouter<'a>,
///     ) {
///         dispatcher.get_observable(MyStruct::EVENT_TYPE, "InteractionCtx").subscribe(self);
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
    
    /// @returns the registerable type, applications type, name, and description
    fn get_info(&self) -> (RegisterableType, ApplicationCommandType, &'static str, Option<&'static str>) {
        (RegisterableType::Event, ApplicationCommandType::ChatInput, "", None)
    }

    fn get_name(&self) -> &'static str {
        self.get_info().2
    }

    fn get_options(&self) -> Vec<ApplicationCommandOption> {
        vec![]
    }
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
pub trait SubHandler: Sync {
    async fn handler(&self, _: InteractionCtx);
}

#[async_trait]
pub trait SubCommandGroup<'a>: SubHandler {}

#[async_trait]
pub trait SubCommand<'a>: SubHandler { }

pub trait SubRegisterable<'a>: SubHandler + Registerable<'a> {}

#[async_trait]
pub trait CommandHandler<'a> {
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
    /// By default, this function will route the interaction down to any subcommands. If
    /// this function doesn't have any subcommands to route down to, it will panic.
    /// @param ctx The context of the interaction.
    async fn handler(&self, ctx: InteractionCtx) {
        if self.get_subs().is_empty() {
            panic!("Command Handler for {} is not implimented!", Self::COMMAND_NAME);
        }
        self.route_down(ctx).await;
    }

    /// This function is called when the command is registered, and also every time a command
    /// is routed down to the appropriate subcommand.
    fn get_subs(&self) -> Vec<&'a dyn SubRegisterable<'a>> {
        Vec::new()
    }

    async fn route_down(&self, ictx: InteractionCtx) {
        let sub: Vec<_> = ictx
        .data.as_ref().expect("Interaction has no data!")
        .options.as_ref().expect("Interaction has no subroutes!")
        .iter().filter(|opt| {
            opt.type_ == ApplicationCommandOptionType::SubCommandGroup ||
            opt.type_ == ApplicationCommandOptionType::SubCommand
        }).collect();
        
        if sub.is_empty() {
            panic!("Expected subroutes, but interaction did not reply with any!");
        } else if sub.len() > 1 {
            panic!("Expected only one subroute, but interaction replied with more than one!");
        } else {
            let s = sub.get(0).unwrap();
            let subs = self.get_subs();
            let handler = 
                subs.iter()
                .find(|h|h.get_name() == s.name.as_str())
                .unwrap_or_else(|| panic!("Sub-Route {} not found!", s.name));
            
            let sub_ctx = InteractionCtx {
                data: Some(InteractionData {
                    options: s.options.clone(),
                    name: s.name.clone(),
                    ..ictx.data.clone().unwrap()
                }),
                ..ictx
            };
            handler.handler(sub_ctx).await;
        }
    }
}

/// Makes the user only able to use structs that implement CommandArg in their EventHandler
pub trait CommandArg {}
