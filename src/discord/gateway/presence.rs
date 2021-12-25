use crate::discord::{resources::user::User, snowflake::Snowflake};

use super::activity::Activity;

/**
 * Presence Update Event
 * @docs https://discord.com/developers/docs/topics/gateway#presence-update
 */
pub struct PresenceUpdateEvent {
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
pub struct ClientStatus {
    /// the user's status set for an active desktop (Windows, Linux, Mac) application session
    pub desktop: Option<String>,
    /// the user's status set for an active mobile (iOS, Android) application session
    pub mobile: Option<String>,
    /// the user's status set for an active web (browser, bot account) application session
    pub web: Option<String>,
}
