use crate::discord::snowflake::Snowflake;

use super::typing::ChannelType;

/**
 * Channel Mention Object
 * @docs https://discord.com/developers/docs/resources/channel#channel-mention-object
 */
pub struct ChannelMention {
    /// id of the channel
    id: Snowflake,
    /// id of the guild containing the channel
    guild_id: Snowflake,
    /// the type of channel
    type_: ChannelType,
    /// the name of the channel
    name: String,
}
