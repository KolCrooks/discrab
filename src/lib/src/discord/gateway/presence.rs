use discordrs_codegen::CommandArg;
use serde::{Deserialize, Serialize};

use crate::{
    core::abstraction::commands::CommandArg,
    discord::{resources::user::User, snowflake::Snowflake},
};

use super::activity::Activity;

/**
 * Presence Update Event
 * @docs https://discord.com/developers/docs/topics/gateway#presence-update
 */
#[derive(Serialize, Deserialize, Clone, CommandArg)]
pub struct PresenceUpdate {
    /// The user presence is being updated for
    pub user: User,
    /// id of the guild
    pub guild_id: Snowflake,
    /// either "idle", "dnd", "online", or "offline"
    pub status: String,
    /// user's current activities
    pub activities: Vec<Activity>,
    /// user's platform-dependent status
    pub client_status: ClientStatus,
}

/**
 * Client Status Object
 * Active sessions are indicated with an "online", "idle", or "dnd" string per platform. If a user is offline or invisible, the corresponding field is not present.
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientStatus {
    /// the user's status set for an active desktop (Windows, Linux, Mac) application session
    pub desktop: Option<String>,
    /// the user's status set for an active mobile (iOS, Android) application session
    pub mobile: Option<String>,
    /// the user's status set for an active web (browser, bot account) application session
    pub web: Option<String>,
}
