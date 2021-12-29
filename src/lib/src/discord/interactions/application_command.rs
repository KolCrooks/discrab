use crate::{
    core::http::rate_limit_client::{send_request, RequestRoute},
    resources::channel::typing::ChannelType,
    util::error::Error,
    Context, Snowflake,
};
use hyper::{Body, Method, Request};
use serde::{self, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * Application Command Structure
 * @docs https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-structure
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationCommand {
    /// The id of the command
    pub id: Snowflake,
    /// The type of command
    #[serde(rename = "type")]
    pub type_: ApplicationCommandType,
    /// The id of the parent application
    pub application_id: u64,
    /// The id of the guild the command is for
    pub guild_id: Option<u64>,
    /// The name of the command
    pub name: String,
    /// The description of the command
    pub description: Option<String>,
    /// The options of the command
    pub options: Option<Vec<ApplicationCommandOption>>,
    /// Whether the command is enabled by default when the app is added to a guild
    pub default_permission: bool,
    /// The version of the command
    pub version: Snowflake,
}

/**
 * Application Command Types
 * @docs https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

/**
 * Application Command Option Structure
 * @docs https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationCommandOption {
    /// The type of option
    #[serde(rename = "type")]
    pub type_: ApplicationCommandOptionType,
    /// 1-32 character name
    pub name: String,
    /// 1-100 character description
    pub description: Option<String>,
    /// if the parameter is required or optional--default false
    pub required: bool,
    /// choices for STRING, INTEGER, and NUMBER types for the user to pick from, max 25
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    /// is a subcommand or subcommand group type, these nested options will be the parameters
    pub options: Option<Vec<ApplicationCommandOption>>,
    /// if the option is a channel type, the channels shown will be restricted to these types
    pub channel_types: Option<Vec<ChannelType>>,
    /// if the option is an INTEGER or NUMBER type, the minimum value permitted
    pub min_value: Option<u64>,
    /// if the option is an INTEGER or NUMBER type, the maximum value permitted
    pub max_value: Option<u64>,
    /// enable autocomplete interactions for this option
    pub autocomplete: bool,
}

/**
 * Application Command Option Type
 * @docs https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-type
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    /// Any integer between -2^53 and 2^53
    Integer = 4,
    Boolean = 5,
    User = 6,
    /// Includes all channel types + categories
    Channel = 7,
    Role = 8,
    /// Includes users and roles
    Mentionable = 9,
    /// Any double between -2^53 and 2^53
    Number = 10,
}

/**
 * Application Command Option Choice Structure
 * @docs https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationCommandOptionChoice {
    /// 1-100 character name
    pub name: String,
    /// value of the choice, up to 100 characters if string
    pub value: ApplicationCommandOptionChoiceValue,
}

/**
 * Application Command Option Choice Value
 */
#[derive(Serialize, Deserialize, Clone)]
pub enum ApplicationCommandOptionChoiceValue {
    String(String),
    Integer(i64),
    Number(f64),
}

impl ApplicationCommand {
    pub async fn get(ctx: Context, id: Snowflake) -> Result<ApplicationCommand, Error> {
        let route = RequestRoute {
            base_route: "/gateway".to_string(),
            major_param: "".to_string(),
        };
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri("https://discord.com/api/gateway/bot")
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        send_request(ctx, route, request_builder).await
    }
}
