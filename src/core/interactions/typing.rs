use crate::discord::{guild::guild_member::GuildMember, snowflake::Snowflake, user::User};

pub struct GuildInteraction {
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
    /// Guild member data for the invoking user, including permissions
    member: Option<GuildMember>,
    /// A continuation token for responding to the interaction
    token: String,
    /// Read-only property, always 1
    version: u32,
    /// For components, the message they were attached to
    message: Option<Message>,
}

pub struct GuildInteraction {
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
    /// User object for the invoking user, if invoked in a DM
    user: Option<User>,
    /// A continuation token for responding to the interaction
    token: String,
    /// Read-only property, always 1
    version: u32,
    /// For components, the message they were attached to
    message: Option<Message>,
}
