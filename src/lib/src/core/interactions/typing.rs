use bitflags::bitflags;
use discrab_codegen::CommandArg;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    api::{channel::{attachment::Attachment, embed::Embed}, ApplicationCommandOptionValue},
    api::ApplicationCommandType,
    core::abstraction::traits::CommandArg,
    discord::{
        interactions::application_command::{
            ApplicationCommandOptionChoice, ApplicationCommandOptionType,
        },
        resources::{channel::message::Message, guild::guild_member::GuildMember, user::User},
        snowflake::Snowflake,
    },
};

use super::message::MessageComponent;

/**
 * Interaction
 * @docs <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object>
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
 * @docs <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data-structure>
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
 * @docs <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-interaction-data-option-structure>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct InteractionDataOption {
    /// The name of the parameter
    pub name: String,
    /// The type of the parameter
    #[serde(rename = "type")]
    pub type_: ApplicationCommandOptionType,
    /// The value of the option resulting from user input
    pub value: Option<ApplicationCommandOptionValue>,
    /// Present if this option is a group or subcommand
    pub options: Option<Vec<InteractionDataOption>>,
    /// true if this option is the currently focused option for autocomplete
    pub focused: Option<bool>,
}

/**
 * Interaction Response Structure
 * @docs <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-response-structure>
 */
#[derive(Clone, Deserialize, Serialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub type_: InteractionCallbackType,
    pub data: Option<InteractionCallbackData>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InteractionCallbackData {
    Message(MessageData),
    Autocomplete(AutocompleteData),
}

impl InteractionCallbackData {
    pub fn message_from_str(msg: String) -> Self {
        InteractionCallbackData::Message(MessageData {
            content: Some(msg),
            tts: None,
            embeds: None,
            allowed_mentions: None,
            flags: None,
            components: None,
            attachments: None,
        })
    }
}

/**
 * Interaction Callback Type
 * @docs <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-autocomplete>
 */
#[derive(Clone, Deserialize, Serialize)]
pub struct AutocompleteData {
    /// autocomplete choices (max of 25 choices)
    pub choices: ApplicationCommandOptionChoice,
}

/**
 * Messages
 * Not all message fields are currently supported.
 * @docs <https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-messages>
 */
#[derive(Clone, Deserialize, Serialize)]
pub struct MessageData {
    /// is the response TTS
    pub tts: Option<bool>,
    /// message content
    pub content: Option<String>,
    /// supports up to 10 embeds
    pub embeds: Option<Vec<Embed>>,
    /// allowed mentions object
    pub allowed_mentions: Option<AllowedMentions>,
    /// interaction callback data flags
    pub flags: Option<u64>,
    /// message components
    pub components: Option<Vec<MessageComponent>>,
    /// attachment objects with filename and description
    pub attachments: Option<Vec<Attachment>>,
}

bitflags! {
    /// Interaction Callback Data Flags
    /// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-flags
    pub struct MessageDataFlags: u64 {
        /// only the user receiving the message can see it
        const EPHEMERAL = 1 << 6;
    }
}

/**
 * Allowed Mention Types
 * @docs <https://discord.com/developers/docs/resources/channel#allowed-mentions-object-allowed-mention-types>
 */
#[derive(Clone, Deserialize, Serialize)]
pub enum AllowedMentionType {
    #[serde(rename = "roles")]
    Roles,
    #[serde(rename = "users")]
    Users,
    #[serde(rename = "everyone")]
    Everyone,
}

/**
 * Allowed Mention Object
 * @docs <https://discord.com/developers/docs/resources/channel#allowed-mentions-object>
 */
#[derive(Clone, Deserialize, Serialize)]
pub struct AllowedMentions {
    /// An array of allowed mention types to parse from the content.
    pub parse: Vec<String>,
    /// Array of role_ids to mention (Max size of 100)
    pub roles: Vec<Snowflake>,
    /// Array of user_ids to mention (Max size of 100)
    pub users: Vec<Snowflake>,
    /// For replies, whether to mention the author of the message being replied to (default false)
    pub replied_user: bool,
}

/**
 * Interaction Callback Type
 */
#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum InteractionCallbackType {
    /// ACK a Ping
    Pong = 1,
    /// respond to an interaction with a message
    ChannelMessageWithSource = 4,
    /// ACK an interaction and edit a response later, the user sees a loading state
    DeferredChannelMessageWithSource = 5,
    /// ACK an interaction and edit the original message later; the user does not see a loading state
    DeferredUpdateMessage = 6,
    /// for components, ACK an interaction and edit the original message later; the user does not see a loading state
    UpdateMessage = 7,
    /// respond to an autocomplete interaction with suggested choices
    ApplicationCommandAutocompleteResult = 8,
}
