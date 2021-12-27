use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::snowflake::Snowflake;

/**
 * Stage Instance Structure
 * @docs https://discord.com/developers/docs/resources/stage-instance#stage-instance-object
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct StageInstance {
    /// The id of this Stage instance
    pub id: Snowflake,
    /// The guild id of the associated Stage channel
    pub guild_id: Snowflake,
    /// The id of the associated Stage channel
    pub channel_id: Snowflake,
    /// The topic of the Stage instance (1-120 characters)
    pub topic: String,
    /// The privacy level of the Stage instance
    pub privacy_level: i64,
    /// Whether or not Stage Discovery is disabled
    pub discoverable_disabled: bool,
}

/**
 * Privacy Level
 * @docs https://discord.com/developers/docs/resources/stage-instance#stage-instance-object-privacy-level
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum PrivacyLevel {
    /// The Stage instance is visible publicly, such as on Stage Discovery.
    Public = 1,
    /// The Stage instance is visible to only guild members.
    GuildOnly = 2,
}
