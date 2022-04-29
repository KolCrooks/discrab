use std::{sync::Arc, hash::{Hash, Hasher}};

use crate::{
    api::{application::Application, channel::typing::ChannelType, Snowflake},
    core::{http::rate_limit_client::{send_request, RequestRoute, send_request_noparse}},
    util::error::Error,
    Context, BASE_URL, SubRegisterable,
};
use hyper::{Body, Method, Request};
use serde::{self, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * Application Command Structure
 * @docs <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-structure>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationCommand {
    /// The id of the command
    pub id: Snowflake,
    /// The type of command
    #[serde(rename = "type")]
    pub type_: ApplicationCommandType,
    /// The id of the parent application
    pub application_id: Snowflake,
    /// The id of the guild the command is for
    pub guild_id: Option<Snowflake>,
    /// The name of the command
    pub name: String,
    /// The description of the command
    pub description: Option<String>,
    /// The options of the command
    pub options: Option<Vec<ApplicationCommandOption>>,
    /// Whether the command is enabled by default when the app is added to a guild
    pub default_permission: bool,
    /// Set of permissions represented as a bit set
    pub default_member_permissions: Option<String>,
    /// Indicates whether the command is available in DMs with the app, only for globally-scoped commands. By default, commands are visible
    pub dm_permission: Option<bool>,
    /// The version of the command
    pub version: Snowflake,
}

/**
 * Application Command Types
 * @docs <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types>
 */
#[derive(Serialize_repr, Deserialize_repr, Clone, PartialEq)]
#[repr(u8)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

fn default_false() -> bool {
    false
}

/**
 * Application Command Option Structure
 * @docs <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure>
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationCommandOption {
    /// The type of option
    #[serde(rename = "type")]
    pub type_: ApplicationCommandOptionType,
    /// 1-32 character name
    pub name: String,
    /// 1-100 character description
    pub description: String,
    /// if the parameter is required or optional--default false
    #[serde(default = "default_false")]
    pub required: bool,
    /// choices for STRING, INTEGER, and NUMBER types for the user to pick from, max 25
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    /// is a subcommand or subcommand group type, these nested options will be the parameters
    pub options: Option<Vec<ApplicationCommandOption>>,
    /// if the option is a channel type, the channels shown will be restricted to these types
    pub channel_types: Option<Vec<ChannelType>>,
    /// if the option is an INTEGER or NUMBER type, the minimum value permitted
    pub min_value: Option<f64>,
    /// if the option is an INTEGER or NUMBER type, the maximum value permitted
    pub max_value: Option<f64>,
    /// enable autocomplete interactions for this option
    #[serde(default = "default_false")]
    pub autocomplete: bool,
}

impl Hash for ApplicationCommandOption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_.hash(state);
        self.name.hash(state);
        self.description.hash(state);
        self.required.hash(state);
        self.choices.hash(state);
        self.options.hash(state);
        self.channel_types.hash(state);
        self.min_value.map(f64::to_bits).hash(state);
        self.max_value.map(f64::to_bits).hash(state);
        self.autocomplete.hash(state);
    }
}

impl Default for ApplicationCommandOption {
    fn default() -> Self {
        Self {
            type_: ApplicationCommandOptionType::Boolean,
            name: "".to_string(),
            description: "".to_string(),
            required: false,
            choices: None,
            options: None,
            channel_types: None,
            min_value: None,
            max_value: None,
            autocomplete: false,
        }
    }
}

impl From<&Arc<dyn SubRegisterable>> for ApplicationCommandOption {
    fn from(sub: &Arc<dyn SubRegisterable>) -> Self {
        let sub_options = sub.get_options();
        ApplicationCommandOption {
            name: sub.get_name().unwrap().to_string(),
            description: sub.get_description().unwrap().to_string(),
            type_: sub.get_reg_type().into(),
            options: if sub_options.is_empty() {
                None
            } else {
                Some(sub_options)
            },
            ..Default::default()
        }
    }
}

impl From<Arc<dyn SubRegisterable>> for ApplicationCommandOption {
    fn from(sub: Arc<dyn SubRegisterable>) -> Self {
        let sub_options = sub.get_options();
        ApplicationCommandOption {
            name: sub.get_name().unwrap().to_string(),
            description: sub.get_description().unwrap().to_string(),
            type_: sub.get_reg_type().into(),
            options: if sub_options.is_empty() {
                None
            } else {
                Some(sub_options)
            },
            ..Default::default()
        }
    }
}


/**
 * Application Command Option Type
 * @docs <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-type>
 */
#[derive(Serialize_repr, Deserialize_repr, Clone, PartialEq, Debug, Eq, Hash)]
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
 * @docs <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure>
 */
#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct ApplicationCommandOptionChoice {
    /// 1-100 character name
    pub name: String,
    /// value of the choice, up to 100 characters if string
    pub value: ApplicationCommandOptionValue,
}

impl ApplicationCommandOptionChoice {
    pub fn new(name: String, value: ApplicationCommandOptionValue) -> Self {
        Self { name, value }
    }
    pub fn new_str(name: String, value: String) -> Self {
        Self { name, value: ApplicationCommandOptionValue::String(value) }
    }
    pub fn new_int(name: String, value: i64) -> Self {
        Self { name, value: ApplicationCommandOptionValue::Integer(value) }
    }
    pub fn new_num(name: String, value: f64) -> Self {
        Self { name, value: ApplicationCommandOptionValue::Number(value) }
    }
}

/**
 * Application Command Option Choice Value
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ApplicationCommandOptionValue {
    String(String),
    Integer(i64),
    Number(f64),
}

impl Hash for ApplicationCommandOptionValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ApplicationCommandOptionValue::String(s) => s.hash(state),
            ApplicationCommandOptionValue::Integer(i) => i.hash(state),
            ApplicationCommandOptionValue::Number(n) => n.to_bits().hash(state),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
/**
 * @see <https://discord.com/developers/docs/interactions/application-commands#create-application-command>
 */
pub struct CreateApplicationCommand {
    /// The name of the command
    pub name: String,
    /// The description of the command
    pub description: String,
    /// The options of the command
    pub options: Option<Vec<ApplicationCommandOption>>,
    /// Whether the command is enabled by default when the app is added to a guild
    pub default_permission: Option<bool>,
    /// Set of permissions represented as a bit set
    pub default_member_permissions: Option<String>,
    /// The type of command
    #[serde(rename = "type")]
    pub type_: Option<ApplicationCommandType>,
}

/**
 * Application Command Edit Structure
 * @docs <https://discord.com/developers/docs/interactions/application-commands#edit-global-application-command>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct EditApplicationCommand {
    /// 1-32 character name
    pub name: Option<String>,
    /// 1-100 character description
    pub description: Option<String>,
    /// the parameters for the command
    pub options: Option<Vec<ApplicationCommandOption>>,
    /// Set of permissions represented as a bit set
    pub default_member_permissions: Option<String>,
    /// Indicates whether the command is available in DMs with the app, only for globally-scoped commands. By default, commands are visible.
    pub dm_permission: Option<bool>,
    /// Replaced by default_member_permissions and will be deprecated in the future. Indicates whether the command is enabled by default when the app is added to a guild.
    pub default_permission: Option<bool>,
}

impl ApplicationCommand {
    /// Gets a global application command
    /// @param id The id of the command
    pub async fn get_global(ctx: Context, id: Snowflake) -> Result<ApplicationCommand, Error> {
        let slf = Application::get_self(ctx.clone()).await?;

        let route = RequestRoute {
            base_route: "/applications/{application.id}/commands/{command.id}/".to_string(),
            major_param: "".to_string(),
        };
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(format!(
                "{}/applications/{}/commands/{}/",
                BASE_URL, slf.id, id
            ))
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        send_request(ctx, route, request_builder).await
    }

    /// Lists the global application commands associated with the application
    pub async fn list_global(ctx: Context) -> Result<Vec<ApplicationCommand>, Error> {
        let slf = Application::get_self(ctx.clone()).await?;

        let route = RequestRoute {
            base_route: "/applications/{application.id}/commands".to_string(),
            major_param: "".to_string(),
        };
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(format!("{}/applications/{}/commands", BASE_URL, slf.id))
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        send_request(ctx, route, request_builder).await
    }

    /**
     * Creates a global application command
     *
     * Creating a command with the same name as an existing command for your application will overwrite the old command.
     * Create a new global command. New global commands will be available in all guilds after 1 hour. Returns 201 and an application command object.
     * @param payload Payload of information for the command
     */
    pub async fn create_global(
        ctx: Context,
        payload: CreateApplicationCommand,
    ) -> Result<ApplicationCommand, Error> {
        let slf = Application::get_self(ctx.clone()).await?;

        let route = RequestRoute {
            base_route: "/applications/{application.id}/commands".to_string(),
            major_param: "".to_string(),
        };
        let request_builder = Request::builder()
            .method(Method::POST)
            .uri(format!("{}/applications/{}/commands", BASE_URL, slf.id))
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        send_request(ctx, route, request_builder).await
    }

    /**
     * Edits a global application command
     *
     * Edit a global command. Updates will be available in all guilds after 1 hour.
     * Returns 200 and an application command object. All fields are optional, but
     * any fields provided will entirely overwrite the existing values of those fields.
     * 
     * @param payload Payload of information for the command
     */
    pub async fn edit_global(
        ctx: Context,
        id: Snowflake,
        payload: EditApplicationCommand,
    ) -> Result<(), Error> {
        let slf = Application::get_self(ctx.clone()).await?;

        let route = RequestRoute {
            base_route: "/applications/{application.id}/commands/{}".to_string(),
            major_param: "".to_string(),
        };
        let request_builder = Request::builder()
            .method(Method::PATCH)
            .uri(format!("{}/applications/{}/commands/{}", BASE_URL, slf.id, id))
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        send_request_noparse(ctx, route, request_builder).await
    }
}
