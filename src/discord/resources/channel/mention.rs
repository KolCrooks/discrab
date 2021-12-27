use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;

use super::typing::ChannelType;

/**
 * Channel Mention Object
 * @docs https://discord.com/developers/docs/resources/channel#channel-mention-object
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ChannelMention {
    /// id of the channel
    pub id: Snowflake,
    /// id of the guild containing the channel
    pub guild_id: Snowflake,
    /// the type of channel
    #[serde(rename = "type")]
    pub type_: ChannelType,
    /// the name of the channel
    pub name: String,
}
