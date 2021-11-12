use std::fmt::{Display, Error, Formatter};

use crate::discord::emoji::Emoji;

/**
 * Channel Types
 * @docs https://discord.com/developers/docs/resources/channel#channel-object-channel-types
 */
pub enum ChannelType {
    /// A text channel within a server
    GUILD_TEXT = 0,
    /// A direct message between users
    DM = 1,
    /// A voice channel within a server
    GUILD_VOICE = 2,
    /// A direct message between multiple users
    GROUP_DM = 3,
    /// An organizational category that contains up to 50 channels
    GUILD_CATEGORY = 4,
    /// A channel that users can follow and crosspost into their own server
    GUILD_NEWS = 5,
    /// A channel in which game developers can sell their game on Discord
    GUILD_STORE = 6,
    /// A temporary sub-channel within a GUILD_NEWS channel
    GUILD_NEWS_THREAD = 10,
    /// A temporary sub-channel within a GUILD_TEXT channel
    GUILD_PUBLIC_THREAD = 11,
    /// A temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
    GUILD_PRIVATE_THREAD = 12,
    /// A voice channel for hosting events with an audience
    GUILD_STAGE_VOICE = 13,
}

/**
 * Message Types
 * @docs https://discord.com/developers/docs/resources/channel#message-object-message-types
 */
pub enum MessageType {
    DEFAULT = 0,
    RECIPIENT_ADD = 1,
    RECIPIENT_REMOVE = 2,
    CALL = 3,
    CHANNEL_NAME_CHANGE = 4,
    CHANNEL_ICON_CHANGE = 5,
    CHANNEL_PINNED_MESSAGE = 6,
    GUILD_MEMBER_JOIN = 7,
    USER_PREMIUM_GUILD_SUBSCRIPTION = 8,
    USER_PREMIUM_GUILD_SUBSCRIPTION_TIER_1 = 9,
    USER_PREMIUM_GUILD_SUBSCRIPTION_TIER_2 = 10,
    USER_PREMIUM_GUILD_SUBSCRIPTION_TIER_3 = 11,
    CHANNEL_FOLLOW_ADD = 12,
    GUILD_DISCOVERY_DISQUALIFIED = 14,
    GUILD_DISCOVERY_REQUALIFIED = 15,
    GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING = 16,
    GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING = 17,
    THREAD_CREATED = 18,
    REPLY = 19,
    CHAT_INPUT_COMMAND = 20,
    THREAD_STARTER_MESSAGE = 21,
    GUILD_INVITE_REMINDER = 22,
    CONTEXT_MENU_COMMAND = 23,
}
