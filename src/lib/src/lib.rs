pub mod core;
pub mod discord;
mod util;

pub use crate::core::abstraction::{
    bot::{Bot, BotBuilder},
    commands::Registerable,
    context::Context,
    event_dispatcher::{EventDispatcher, Events},
};

pub use discordrs_codegen;
