use serde::{Deserialize, Serialize};

use crate::discord::{
    resources::{channel::message::Message, guild::guild_member::GuildMember, user::User},
    snowflake::Snowflake,
};

/**
 * Interaction
 * @docs https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Interaction {
    /// The id of the interaction
    pub id: Snowflake,
    /// The id of the application this interaction is for
    pub application_id: Snowflake,
    /// The type of interaction
    pub interaction_type: String,
    /// The command data payload
    pub data: Option<String>,
    /// The guild it was sent from
    pub guild_id: Option<String>,
    /// The channel it was sent from
    pub channel_id: Option<Snowflake>,
    /// Guild member data for the invoking user, including permissions, if invoked in a guild
    pub member: Option<GuildMember>,
    /// User object for the invoking user, if invoked in a DM
    pub user: Option<User>,
    /// A continuation token for responding to the interaction
    pub token: String,
    /// Read-only property, always 1
    pub version: u32,
    /// For components, the message they were attached to
    pub message: Option<Box<Message>>,
}
