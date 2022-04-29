use std::{sync::Arc, panic::{UnwindSafe, RefUnwindSafe}};

use async_trait::async_trait;

use crate::{api::{ApplicationCommandOptionType, ApplicationCommandType, ApplicationCommandOption}, Events, EventDispatcher, InteractionRouter, Context, events::InteractionCtx};

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
pub trait Registerable {
    fn get_reg_type(&self) -> RegisterableType {
        RegisterableType::Event
    }

    fn get_event_type(&self) -> Option<Events> {
        None
    }
    
    fn get_application_command_type(&self) -> Option<ApplicationCommandType> {
        None
    }
    
    fn get_name(&self) -> Option<&'static str> {
        None
        
    }

    fn get_description(&self) -> Option<&'static str> {
        None
    }

    fn get_options(&self) -> Vec<ApplicationCommandOption> {
        vec![]
    }
}

pub trait RegFns {
    fn reg_event(self: &Arc<Self>, _: &mut EventDispatcher) {}
    fn reg_command(self: &Arc<Self>, _: Context, _: Arc<InteractionRouter>) {}
}

#[async_trait]
pub trait CommonHandler {
    async fn handler(&self, _: InteractionCtx);
}

pub trait SubRegisterable: CommonHandler + Registerable + Send + Sync + RefUnwindSafe + UnwindSafe {}

pub type SubsVector = Vec<Arc<dyn SubRegisterable>>;


#[macro_export]
macro_rules! registerable_list {
    ($($x:expr),+ $(,)?) => {
        vec![$(std::sync::Arc::new($x)),+]
    };
}