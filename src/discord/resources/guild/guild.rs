use crate::discord::{
    gateway::presence::PresenceUpdateEvent,
    resources::{channel::Channel, emoji::Emoji, sticker::Sticker, voice::VoiceState},
    snowflake::Snowflake,
};

use super::{
    guild_member::GuildMember, role::Role, stage_instance::StageInstance,
    welcome_screen::WelcomeScreen,
};

/**
 * Guild Structure
 * @docs https://discord.com/developers/docs/resources/guild#guild-object-guild-structure
 */
pub struct Guild {
    /// guild id
    pub id: Snowflake,
    /// guild name (2-100 characters, excluding trailing and leading whitespace)
    pub name: String,
    /// icon hash
    pub icon: Option<String>,
    /// icon hash, returned when in the template object
    pub icon_hash: Option<String>,
    /// splash hash
    pub splash: Option<String>,
    /// discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
    pub discovery_splash: Option<String>,
    /// true if the user is the owner of the guild
    pub owner: bool,
    /// id of owner
    pub owner_id: Snowflake,
    /// total permissions for the user in the guild (excludes overwrites)
    pub permissions: Option<String>,
    /// voice region id for the guild (deprecated)
    pub region: Option<String>,
    /// id of afk channel
    pub afk_channel_id: Option<Snowflake>,
    /// afk timeout in seconds
    pub afk_timeout: i64,
    /// true if the server widget is enabled
    pub widget_enabled: bool,
    /// the channel id that the widget will generate an invite to, or null if set to no invite
    pub widget_channel_id: Option<Snowflake>,
    /// verification level required for the guild
    pub verification_level: i64,
    /// default message notifications level
    pub default_message_notifications: i64,
    /// explicit content filter level
    pub explicit_content_filter: i64,
    /// roles in the guild
    pub roles: Vec<Role>,
    /// custom guild emojis
    pub emojis: Vec<Emoji>,
    /// enabled guild features
    pub features: Vec<String>,
    /// required MFA level for the guild
    pub mfa_level: i64,
    /// application id of the guild creator if it is bot-created
    pub application_id: Option<Snowflake>,
    /// the id of the channel where guild notices such as welcome messages and boost events are posted
    pub system_channel_id: Option<Snowflake>,
    /// system channel flags
    pub system_channel_flags: i64,
    /// the id of the channel where Community guilds can display rules and/or guidelines
    pub rules_channel_id: Option<Snowflake>,
    /// when this guild was joined at
    pub joined_at: Option<String>,
    /// true if this is considered a large guild
    pub large: bool,
    /// true if this guild is unavailable due to an outage
    pub unavailable: bool,
    /// total number of members in this guild
    pub member_count: Option<i64>,
    /// states of members currently in voice channels; lacks the guild_id key
    pub voice_states: Option<Vec<VoiceState>>,
    /// users in the guild
    pub members: Option<Vec<GuildMember>>,
    /// channels in the guild
    pub channels: Option<Vec<Channel>>,
    /// all active threads in the guild that current user has permission to view
    pub threads: Option<Vec<Channel>>,
    /// presences of the members in the guild, will only include non-offline members if the size is greater than large threshold
    pub presences: Option<Vec<PresenceUpdateEvent>>,
    /// the maximum number of presences for the guild (null is always returned, apart from the largest of guilds)
    pub max_presences: Option<i64>,
    /// the maximum number of members for the guild
    pub max_members: i64,
    /// the vanity url code for the guild
    pub vanity_url_code: Option<String>,
    /// the description of a Community guild
    pub description: Option<String>,
    /// banner hash
    pub banner: Option<String>,
    /// premium tier (Server Boost level)
    pub premium_tier: i64,
    /// the number of boosts this guild currently has
    pub premium_subscription_count: Option<i64>,
    /// the preferred locale of a Community guild; used in server discovery and notices from Discord; defaults to "en-US"
    pub preferred_locale: String,
    /// the id of the channel where admins and moderators of Community guilds receive notices from Discord
    pub public_updates_channel_id: Option<Snowflake>,
    /// the maximum amount of users in a video channel
    pub max_video_channel_users: Option<i64>,
    /// approximate number of members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    pub approximate_member_count: Option<i64>,
    /// approximate number of non-offline members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    pub approximate_presence_count: Option<i64>,
    /// the welcome screen of a Community guild, shown to new members, returned in an Invite's guild object
    pub welcome_screen: Option<WelcomeScreen>,
    /// guild NSFW level
    pub nsfw_level: i64,
    /// Stage instances in the guild
    pub stage_instances: Option<Vec<StageInstance>>,
    /// custom guild stickers
    pub stickers: Option<Vec<Sticker>>,
}

pub struct UnavailableGuild {
    id: Snowflake,
    unavailable: bool,
}
