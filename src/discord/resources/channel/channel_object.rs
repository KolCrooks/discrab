use chrono::{DateTime, Utc};

use crate::discord::{resources::user::User, snowflake::Snowflake};

use super::typing::{
    ChannelType, PermissionsOverwriteObject, ThreadMember, ThreadMetadata, VideoQualityMode,
};

/**
 * Represents a guild or DM channel within Discord.
 * @docs https://discord.com/developers/docs/resources/channel#channel-object-channel-structure
 */
pub struct Channel {
    /// The id of this channel
    pub id: Snowflake,
    /// The type of channel
    pub channel_type: ChannelType,
    /// The id of the guild (may be missing for some channel objects received over gateway guild dispatches)
    pub guild_id: Option<Snowflake>,
    /// Sorting position of the channel
    pub position: Option<u64>,
    /// Explicit permission overwrites for members and roles
    pub permission_overwrites: Option<Vec<PermissionsOverwriteObject>>,
    /// The name of the channel (1-100 characters)
    pub name: Option<String>,
    /// The channel topic (0-1024 characters)
    pub topic: Option<String>,
    /// Whether the channel is nsfw
    pub nsfw: Option<bool>,
    /// The id of the last message sent in this channel (may not point to an existing or valid message)
    pub last_message_id: Option<Snowflake>,
    /// The bitrate (in bits) of the voice channel
    pub bitrate: Option<u64>,
    /// The user limit of the voice channel
    pub user_limit: Option<u64>,
    /// Amount of seconds a user has to wait before sending another message (0-21600); bots, as well as users with the permission manage_messages or manage_channel, are unaffected
    pub rate_limit_per_user: Option<u64>,
    /// The recipients of the DM
    pub recipients: Option<Vec<User>>,
    /// Icon hash
    pub icon: Option<String>,
    /// Id of the creator of the group DM or thread
    pub owner_id: Option<Snowflake>,
    /// Application id of the group DM creator if it is bot-created
    pub application_id: Option<Snowflake>,
    /// The id of the parent category for a channel (each parent category can contain up to 50 channels), for threads: id of the text channel this thread was created
    pub parent_id: Option<Snowflake>,
    /// When the last pinned message was pinned. This may be null in events such as GUILD_CREATE when a message is not pinned.
    pub last_pin_timestamp: Option<DateTime<Utc>>,
    /// Voice region id for the voice channel, automatic when set to null
    pub rtc_region: Option<String>,
    /// The camera video quality mode of the voice channel, 1 when not present
    pub video_quality_mode: Option<VideoQualityMode>,
    /// An approximate count of messages in a thread, stops counting at 50
    pub message_count: Option<u64>,
    /// An approximate count of users in a thread, stops counting at 50
    pub member_count: Option<u64>,
    /// Thread-specific fields not needed by other channels
    pub thread_metadata: Option<ThreadMetadata>,
    /// Thread member object for the current user, if they have joined the thread, only included on certain API endpoints
    pub member: Option<ThreadMember>,
    /// Default duration that the clients (not the API) will use for newly created threads, in minutes, to automatically archive the thread after recent activity, can be set to: 60, 1440, 4320, 10080
    pub default_auto_archive_duration: Option<u64>,
    /// Computed permissions for the invoking user in the channel, including overwrites, only included when part of the resolved data received on a slash command interaction
    pub permissions: Option<String>,
}
