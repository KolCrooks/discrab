use discordrs_codegen::CommandArg;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    core::abstraction::abstraction_traits::CommandArg,
    discord::{
        interactions::application_command::ApplicationCommandOptionType,
        resources::{channel::message::Message, guild::guild_member::GuildMember, user::User},
        snowflake::Snowflake,
    },
    ApplicationCommandType,
};

/**
 * Interaction
 * @docs https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
 */
#[derive(Serialize, Deserialize, Clone, CommandArg)]
pub struct Interaction {
    /// The id of the interaction
    pub id: Snowflake,
    /// The id of the application this interaction is for
    pub application_id: Snowflake,
    /// The type of interaction
    #[serde(rename = "type")]
    pub type_: InteractionType,
    /// The command data payload
    pub data: Option<InteractionData>,
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

/**
 * Interaction Type
*/
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
}

/**
 * Interaction Data Structure
 * @docs https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data-structure
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct InteractionData {
    /// The id of the invoked command
    pub id: Snowflake,
    /// The name of the invoked command
    pub name: String,
    /// The type of the invoked command
    #[serde(rename = "type")]
    pub type_: ApplicationCommandType,
    /// The params + values from the user
    pub options: Option<Vec<InteractionDataOption>>,
    /// The custom_id of the component
    pub custom_id: Option<String>,
    /// The type of the component
    pub component_type: Option<String>,
    /// The values the user selected
    pub values: Option<Vec<String>>,
    /// The id of user or message targetted by a user or message command
    pub target_id: Option<Snowflake>,
}

/**
 * Application Command Interaction Data Option
 * All options have names, and an option can either be a parameter and input value--in which case value will be set--or it can denote a subcommand or group--in which case it will contain a top-level key and another array of options.
 * @docs https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-interaction-data-option-structure
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct InteractionDataOption {
    /// The name of the parameter
    pub name: String,
    /// The type of the parameter
    #[serde(rename = "type")]
    pub type_: ApplicationCommandOptionType,
    /// The value of the option resulting from user input
    pub value: Option<String>,
    /// Present if this option is a group or subcommand
    pub options: Option<Vec<InteractionDataOption>>,
    /// true if this option is the currently focused option for autocomplete
    pub focused: Option<bool>,
}
