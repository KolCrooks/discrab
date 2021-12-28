use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::{
    gateway::presence::PresenceUpdate,
    resources::{
        application::Application,
        channel::{typing::ThreadMember, Channel},
        emoji::Emoji,
        guild::{
            guild_member::GuildMember,
            integration::{Account, IntegrationType},
            role::Role,
        },
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

/**
 * Guild Ban Add or Remove
 * @docs https://discord.com/developers/docs/topics/gateway#guild-ban-add
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildBanAddRemove {
    /// the id of the guild
    pub guild_id: Snowflake,
    /// the banned/unbanned user
    pub user: User,
}

/**
 * Guild Emojis
 * @docs https://discord.com/developers/docs/topics/gateway#guild-emojis-update
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildEmojisUpdate {
    /// the id of the guild
    pub guild_id: Snowflake,
    /// array of emojis
    pub emojis: Vec<Emoji>,
}

/**
 * Guild Stickers Update
 * @docs https://discord.com/developers/docs/topics/gateway#guild-stickers-update
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildStickersUpdate {
    /// the id of the guild
    pub guild_id: Snowflake,
    /// array of stickers
    pub stickers: Vec<Emoji>,
}

/**
 * Guild Integrations Update
 * @docs https://discord.com/developers/docs/topics/gateway#guild-integrations-update
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildIntegrationsUpdate {
    /// the id of the guild
    pub guild_id: Snowflake,
}

/**
 * Guild Member Add
 * @docs https://discord.com/developers/docs/topics/gateway#guild-member-add
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildMemberAdd {
    /// The user this guild member represents
    pub user: Option<User>,
    /// this users guild nickname
    pub nick: Option<String>,
    /// the member's guild avatar hash
    pub avatar: Option<String>,
    /// array of role object ids
    pub roles: Vec<Snowflake>,
    /// when the user joined the guild
    pub joined_at: String,
    /// when the user started boosting the guild
    pub premium_since: Option<String>,
    /// whether the user is deafened in voice channels
    pub deaf: bool,
    /// whether the user is muted in voice channels
    pub mute: bool,
    /// whether the user has not yet passed the guild's Membership Screening requirements
    pub pending: Option<bool>,
    /// total permissions of the member in the channel, including overwrites, returned when in the interaction object
    pub permissions: Option<String>,
    /// id of the guild
    pub guild_id: Snowflake,
}

/**
 * Guild Member Remove
 * @docs https://discord.com/developers/docs/topics/gateway#guild-member-remove
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildMemberRemove {
    /// the id of the guild
    pub guild_id: Snowflake,
    /// the user who was removed
    pub user: User,
}

/**
 * Guild Members
 * @docs https://discord.com/developers/docs/topics/gateway#guild-members-chunk
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildMembersChunk {
    /// the id of the guild
    pub guild_id: Snowflake,
    /// set of guild members
    pub members: Vec<GuildMember>,
    /// the chunk index in the expected chunks for this response (0 <= chunk_index < chunk_count)
    pub chunk_index: u64,
    /// the total number of expected chunks for this response
    pub chunk_count: u64,
    /// if passing an invalid id to REQUEST_GUILD_MEMBERS, it will be returned here
    pub not_found: Option<Vec<Snowflake>>,
    /// if passing true to REQUEST_GUILD_MEMBERS, presences of the returned members will be here
    pub presences: Option<Vec<PresenceUpdate>>,
    /// the nonce used in the Guild Members Request
    pub nonce: Option<String>,
}

/**
 * Guild Role Create or Update or Delete
 * @docs https://discord.com/developers/docs/topics/gateway#guild-role-create
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildRoleCreateUpdateDelete {
    /// the id of the guild
    pub guild_id: Snowflake,
    /// the role created
    pub role: Role,
}

/**
 * Guild Scheduled Event User Add or Remove
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildScheduledEventUserAddRemove {
    /// the id of the guild scheduled event
    pub guild_scheduled_event_id: Snowflake,
    /// the id of the user
    pub user_id: Snowflake,
    /// the id of the guild
    pub guild_id: Snowflake,
}

/**
 * Integration Structure
 * @docs https://discord.com/developers/docs/topics/gateway#integrations
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct IntegrationCreateUpdate {
    /// The id of the integration
    pub id: u64,
    /// The name of the integration
    pub name: String,
    /// The type of the integration
    #[serde(rename = "type")]
    pub type_: IntegrationType,
    /// Is this integration enabled
    pub enabled: bool,
    /// Is this integration syncing
    pub syncing: bool,
    /// The id that this integration uses for "subscribers"
    pub role_id: Option<u64>,
    /// Whether emoticons should be synced for this integration (twitch only currently)
    pub enable_emoticons: Option<bool>,
    /// The behavior of expiring subscribers
    pub expire_behavior: Option<String>,
    /// The grace period (in days) before expiring subscribers
    pub expire_grace_period: Option<u64>,
    /// User for this integration
    pub user: Option<User>,
    /// Integration account information
    pub account: Account,
    /// When this integration was last synced
    pub synced_at: Option<String>,
    /// How many subscribers this integration has
    pub subscriber_count: Option<u64>,
    /// Has this integration been revoked
    pub revoked: Option<bool>,
    /// The bot/OAuth2 application for discord integrations
    pub application: Option<Application>,
    /// The guild id for this integration
    pub guild_id: Option<Snowflake>,
}

/**
 * Integration Delete
 * @docs https://discord.com/developers/docs/topics/gateway#integration-delete
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct IntegrationDelete {
    /// The id of the integration
    pub id: u64,
    /// The id of the guild
    pub guild_id: Snowflake,
    /// The id of the bot/OAuth2 application for this discord integration
    pub application_id: Option<Snowflake>,
}

/**
 * Invite Create
 * @docs https://discord.com/developers/docs/topics/gateway#invites
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct InviteCreate {
    /// the channel the invite is for
    pub channel_id: Snowflake,
    /// the unique invite code
    pub code: String,
    /// the time at which the invite was created
    pub created_at: String,
    /// the guild of the invite
    pub guild_id: Option<Snowflake>,
    /// the user that created the invite
    pub inviter: Option<User>,
    /// how long the invite is valid for (in seconds)
    pub max_age: Option<u64>,
    /// the maximum number of times the invite can be used
    pub max_uses: Option<u64>,
    /// the type of target for this voice channel invite
    pub target_type: Option<String>,
    /// the user whose stream to display for this voice channel stream invite
    pub target_user: Option<User>,
    /// the embedded application to open for this voice channel embedded application invite
    pub target_application: Option<Application>,
    /// whether or not the invite is temporary (invited users will be kicked on disconnect unless they're assigned a role)
    pub temporary: Option<bool>,
    /// how many times the invite has been used (always will be 0)
    pub uses: Option<u64>,
}

/**
 * Invite Target
 * @docs https://discord.com/developers/docs/resources/invite#invite-object-invite-target-types
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum InviteTargetType {
    Stream = 1,
    EmbeddedApplication = 2,
}

/**
 * Invite Delete Event
 * @docs https://discord.com/developers/docs/topics/gateway#invite-delete
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct InviteDelete {
    /// the channel of the invite
    pub channel_id: Snowflake,
    /// the guild of the invite
    pub guild_id: Option<Snowflake>,
    /// the unique invite code
    pub code: String,
}

/**
 * Message Delete
 * @docs https://discord.com/developers/docs/topics/gateway#message-delete
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageDelete {
    /// the id of the message
    pub id: Snowflake,
    /// the id of the channel
    pub channel_id: Snowflake,
    /// the id of the guild
    pub guild_id: Option<Snowflake>,
}

/**
 * Message Delete Bulk
 * @docs https://discord.com/developers/docs/topics/gateway#message-delete-bulk
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageDeleteBulk {
    /// the ids of the messages
    pub ids: Vec<Snowflake>,
    /// the id of the channel
    pub channel_id: Snowflake,
    /// the id of the guild
    pub guild_id: Option<Snowflake>,
}

/**
 * Message Reaction Add
 * @docs https://discord.com/developers/docs/topics/gateway#message-reaction-add
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageReactionAdd {
    /// the id of the user
    pub user_id: Snowflake,
    /// the id of the channel
    pub channel_id: Snowflake,
    /// the id of the message
    pub message_id: Snowflake,
    /// the id of the guild
    pub guild_id: Option<Snowflake>,
    /// the member who reacted if this happened in a guild
    pub member: Option<GuildMember>,
    /// the emoji used to react - example
    pub emoji: Emoji,
}

/**
 * Message Reaction Remove
 * @docs https://discord.com/developers/docs/topics/gateway#message-reaction-remove
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageReactionRemove {
    /// the id of the user
    pub user_id: Snowflake,
    /// the id of the channel
    pub channel_id: Snowflake,
    /// the id of the message
    pub message_id: Snowflake,
    /// the id of the guild
    pub guild_id: Option<Snowflake>,
    /// the emoji used to react - example
    pub emoji: Emoji,
}

/**
 * Message Reaction Remove All
 * @docs https://discord.com/developers/docs/topics/gateway#message-reaction-remove-all
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageReactionRemoveAll {
    /// the id of the channel
    pub channel_id: Snowflake,
    /// the id of the message
    pub message_id: Snowflake,
    /// the id of the guild
    pub guild_id: Option<Snowflake>,
}

/**
 * Message Reaction Remove Emoji
 * @docs https://discord.com/developers/docs/topics/gateway#message-reaction-remove-emoji
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageReactionRemoveEmoji {
    /// the id of the channel
    pub channel_id: Snowflake,
    /// the id of the message
    pub message_id: Snowflake,
    /// the id of the guild
    pub guild_id: Option<Snowflake>,
    /// the emoji used to react - example
    pub emoji: Emoji,
}

/**
 * Typing Start
 * @docs https://discord.com/developers/docs/topics/gateway#typing-start
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct TypingStart {
    /// the id of the channel
    pub channel_id: Snowflake,
    /// the id of the guild
    pub guild_id: Option<Snowflake>,
    /// the id of the user
    pub user_id: Snowflake,
    /// unix time (in seconds) of when the user started typing
    pub timestamp: u64,
    /// the member who started typing if this happened in a guild
    pub member: Option<GuildMember>,
}

/**
 * Voice Server Update
 * @docs https://discord.com/developers/docs/topics/gateway#voice-server-update
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct VoiceServerUpdate {
    /// the voice connection token
    pub token: String,
    /// the guild this voice server update is for
    pub guild_id: Snowflake,
    /// the voice server host
    pub endpoint: String,
}

/**
 * Webhooks Update
 * @docs https://discord.com/developers/docs/topics/gateway#webhooks-update
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct WebhooksUpdate {
    /// the id of the guild
    pub guild_id: Snowflake,
    /// the id of the channel
    pub channel_id: Snowflake,
}
