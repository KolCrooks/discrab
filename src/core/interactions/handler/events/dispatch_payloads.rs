use serde::{Deserialize, Serialize};

use crate::discord::{
    resources::{
        channel::{typing::ThreadMember, Channel},
        user::User,
    },
    snowflake::Snowflake,
};

/**
 * Channel Pins Update
 * Sent when a message is pinned or unpinned in a text channel. This is not sent when a pinned message is deleted.
 * @docs https://discord.com/developers/docs/topics/gateway#channel-pins-update
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct ChannelPinsUpdate {
    /// the id of the guild
    pub guild_id: Option<Snowflake>,
    /// the id of the channel
    pub channel_id: Snowflake,
    /// the time at which the most recent pinned message was pinned
    pub last_pin_timestamp: Option<String>,
}

/**
 * Thread List Sync Event Fields
 * @docs https://discord.com/developers/docs/topics/gateway#thread-list-sync-thread-list-sync-event-fields
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ThreadListSync {
    /// the id of the guild
    pub guild_id: Snowflake,
    /// the parent channel ids whose threads are being synced. If omitted, then threads were synced for the entire guild. This array may contain channel_ids that have no active threads as well, so you know to clear that data.
    pub channel_ids: Vec<Snowflake>,
    /// all active threads in the given channels that the current user can access
    pub threads: Vec<Channel>,
    /// all thread member objects from the synced threads for the current user, indicating which threads the current user has been added to
    pub members: Vec<ThreadMember>,
}

/**
 * Thread Member Update
 * @docs https://discord.com/developers/docs/topics/gateway#thread-member-update
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ThreadMemberUpdate {
    /// The id of the thread
    pub id: Snowflake,
    /// The id of the user
    pub user_id: Snowflake,
    /// The time the current user last joined the thread
    pub join_timestamp: String,
    /// Any user-thread settings, currently only used for notifications
    pub flags: u64,
    /// The id of the guild
    pub guild_id: Snowflake,
}

/**
 * Thread Members Update Event Fields
 * @docs https://discord.com/developers/docs/topics/gateway#thread-members-update
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ThreadMembersUpdate {
    /// The id of the thread
    pub id: Snowflake,
    /// The id of the guild
    pub guild_id: Snowflake,
    /// The approximate number of members in the thread, capped at 50
    pub member_count: u64,
    /// The users who were added to the thread
    pub added_members: Vec<ThreadMember>,
    /// The id of the users who were removed from the thread
    pub removed_member_ids: Vec<Snowflake>,
}

/**
 * Guild Member Update Event Fields
 * @docs https://discord.com/developers/docs/topics/gateway#guild-member-update
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildMemberUpdate {
    /// The id of the guild
    pub guild_id: Snowflake,
    /// array of snowflakes user role ids
    pub roles: Vec<Snowflake>,
    /// the user
    pub user: User,
    /// nickname of the user in the guild
    pub nick: Option<String>,
    /// the member's guild avatar hash
    pub avatar: Option<String>,
    /// when the user joined the guild
    pub joined_at: Option<String>,
    /// when the user started boosting the guild
    pub premium_since: Option<String>,
    /// whether the user is deafened in voice channels
    pub deaf: Option<bool>,
    /// whether the user is muted in voice channels
    pub mute: Option<bool>,
    /// whether the user has not yet passed the guild's Membership Screening requirements
    pub pending: Option<bool>,
    /// when the user's timeout will expire and the user will be able to communicate in the guild again, null or a time in the past if the user is not timed out
    pub communication_disabled_until: Option<String>,
}
