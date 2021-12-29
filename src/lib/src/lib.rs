pub mod core;
mod discord;
mod util;

pub use crate::core::abstraction::{
    bot::{Bot, BotBuilder},
    commands::{EventHandler, Registerable},
    context::Context,
    event_dispatcher::{EventDispatcher, Events},
};

pub use discordrs_codegen;

pub mod resources {
    pub use crate::discord::resources::channel::message::Message;
    pub use crate::discord::resources::channel::Channel;
    pub use crate::discord::resources::*;
}

pub use discord::permissions::Permissions;
pub use discord::snowflake::Snowflake;
