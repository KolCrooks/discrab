use discrab_codegen::CommandArg;
use serde::{Deserialize, Serialize};

use crate::{api::Snowflake, core::abstraction::traits::CommandArg};

use super::user::User;

/**
 * Guild Scheduled Event Structure
 * @docs <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event>
 */
#[derive(Serialize, Deserialize, Clone, CommandArg)]
pub struct GuildScheduledEvent {
    /// The id of the scheduled event
    pub id: Snowflake,
    /// The guild id which the scheduled event belongs to
    pub guild_id: Snowflake,
    /// The channel id in which the scheduled event will be hosted, or null if scheduled entity type is EXTERNAL
    pub channel_id: Option<u64>,
    /// The id of the user that created the scheduled event *
    pub creator_id: Option<u64>,
    /// The name of the scheduled event (1-100 characters)
    pub name: String,
    /// The description of the scheduled event (1-1000 characters)
    pub description: Option<String>,
    /// The time the scheduled event will start
    pub scheduled_start_time: String,
    /// The time the scheduled event will end, required if entity_type is EXTERNAL
    pub scheduled_end_time: Option<String>,
    /// The privacy level of the scheduled event
    pub privacy_level: String,
    /// The status of the scheduled event
    pub status: String,
    /// The type of the scheduled event
    pub entity_type: String,
    /// The id of an entity associated with a guild scheduled event
    pub entity_id: Option<u64>,
    /// Additional metadata for the guild scheduled event
    pub entity_metadata: Option<String>,
    /// The user that created the scheduled event
    pub creator: Option<User>,
    /// The number of users subscribed to the scheduled event
    pub user_count: Option<u64>,
}
