use crate::discord::{
    resources::{channel::message::Message, guild::guild_member::GuildMember, user::User},
    snowflake::Snowflake,
};

/**
 * Interaction
 * @docs https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
 */
pub struct Interaction {
    /// The id of the interaction
    id: Snowflake,
    /// The id of the application this interaction is for
    application_id: Snowflake,
    /// The type of interaction
    interaction_type: String,
    /// The command data payload
    data: Option<String>,
    /// The guild it was sent from
    guild_id: Option<String>,
    /// The channel it was sent from
    channel_id: Option<Snowflake>,
    /// Guild member data for the invoking user, including permissions, if invoked in a guild
    member: Option<GuildMember>,
    /// User object for the invoking user, if invoked in a DM
    user: Option<User>,
    /// A continuation token for responding to the interaction
    token: String,
    /// Read-only property, always 1
    version: u32,
    /// For components, the message they were attached to
    message: Option<Box<Message>>,
}
