use chrono::{DateTime, Utc};

use crate::discord::{resources::user::User, snowflake::Snowflake};

/**
 * guild member object
 * @docs https://discord.com/developers/docs/resources/guild#guild-member-object
 */
pub struct GuildMember {
    /// The user this guild member represents
    pub user: Option<User>,
    /// this users guild nickname
    pub nick: Option<String>,
    /// the member's guild avatar hash
    pub avatar: Option<String>,
    /// array of role object ids
    pub roles: Vec<Snowflake>,
    /// when the user joined the guild
    pub joined_at: DateTime<Utc>,
    /// when the user started boosting the guild
    pub premium_since: Option<DateTime<Utc>>,
    /// whether the user is deafened in voice channels
    pub deaf: bool,
    /// whether the user is muted in voice channels
    pub mute: bool,
    /// whether the user has not yet passed the guild's Membership Screening requirements
    pub pending: Option<bool>,
    /// total permissions of the member in the channel, including overwrites, returned when in the interaction object
    pub permissions: Option<String>,
}
