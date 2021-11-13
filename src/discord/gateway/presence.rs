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
