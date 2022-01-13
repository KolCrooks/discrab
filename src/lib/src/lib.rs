pub mod core;
mod discord;
mod util;

pub use crate::core::abstraction::{
    abstraction_traits::{CommandHandler, EventHandler, Registerable},
    bot::Bot,
    context::Context,
    event_dispatcher::{EventDispatcher, Events},
    interaction_router::InteractionRouter,
};

pub mod macros {
    pub use discordrs_codegen::*;
}

/**
 * For internal use only. Is public because it is used by the proc macros
 */
#[doc(hidden)]
pub mod __internal__ {
    pub use crate::core::abstraction::abstraction_traits::InternalEventHandler;
}

/**
 * Discord objects
 */
pub mod api {
    pub use crate::discord::interactions::application_command::{
        ApplicationCommand, ApplicationCommandType,
    };
    pub use crate::discord::permissions::Permissions;
    pub use crate::discord::resources::channel::embed;
    pub use crate::discord::resources::channel::message::Message;
    pub use crate::discord::resources::channel::Channel;
    pub use crate::discord::resources::*;
    pub use crate::discord::snowflake::Snowflake;
}

/**
 * Objects associated with different events
 */
pub mod events {
    pub use crate::api::guild::guild_object::{Guild, UnavailableGuild};
    pub use crate::api::guild::stage_instance::StageInstance;
    pub use crate::api::guild_scheduled_event::GuildScheduledEvent;
    pub use crate::api::user::User;
    pub use crate::api::voice::VoiceState;
    pub use crate::api::{Channel, Message};
    pub use crate::core::interactions::handler::events::dispatch_payloads::*;
    pub use crate::core::interactions::{
        interaction_event::InteractionCreate, typing::Interaction,
    };
    pub use crate::discord::gateway::presence::PresenceUpdate;
}

#[doc(hidden)]
pub(crate) static BASE_URL: &str = "https://discord.com/api/v9";
