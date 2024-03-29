use discrab_codegen::CommandArg;
use hyper::{Body, Method, Request};

use crate::{
    api::{guild::guild_member::GuildMember, user::User, Message, Snowflake, ApplicationCommandOptionValue},
    core::{
        abstraction::traits::CommandArg,
        http::rate_limit_client::{send_request_noparse, RequestRoute},
    },
    util::error::Error,
    Context, BASE_URL,
};

use super::typing::{
    Interaction, InteractionCallbackData, InteractionCallbackType, InteractionData,
    InteractionResponse, InteractionType, InteractionDataOption,
};

#[derive(CommandArg)]
pub struct InteractionCtx {
    // has_resolved: Option,
    /// The id of the interaction
    pub id: Snowflake,
    /// The id of the application this interaction is for
    pub application_id: Snowflake,
    /// The type of interaction
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
    /// internal context object
    pub __ctx__: Context,
}

pub struct InteractionOption<T>{
    pub name: String,
    pub value: T,
}

impl From<InteractionDataOption> for InteractionOption<String> {
    fn from(value: InteractionDataOption) -> Self {
        match value.value.unwrap() {
            ApplicationCommandOptionValue::String(v) => InteractionOption {
                name: value.name,
                value: v,
            },
            _ => panic!("InteractionOption::from() - InteractionDataOption::value is not a String"),
        }
    }
}
impl From<InteractionDataOption> for InteractionOption<i64> {
    fn from(value: InteractionDataOption) -> Self {
        match value.value.unwrap() {
            ApplicationCommandOptionValue::Integer(v) => InteractionOption {
                name: value.name,
                value: v,
            },
            _ => panic!("InteractionOption::from() - InteractionDataOption::value is not an Integer"),
        }
    }
}

impl From<InteractionDataOption> for InteractionOption<f64> {
    fn from(value: InteractionDataOption) -> Self {
        match value.value.unwrap() {
            ApplicationCommandOptionValue::Number(v) => InteractionOption {
                name: value.name,
                value: v,
            },
            _ => panic!("InteractionOption::from() - InteractionDataOption::value is not an Float"),
        }
    }
}

impl InteractionCtx {
    /// Creates a new InteractionCtx object from an Interaction
    pub fn from_interaction(ctx: Context, int: Interaction) -> Self {
        Self {
            __ctx__: ctx,
            application_id: int.application_id,
            channel_id: int.channel_id,
            data: int.data,
            guild_id: int.guild_id,
            id: int.id,
            member: int.member,
            message: int.message,
            token: int.token,
            type_: int.type_,
            user: int.user,
            version: int.version,
        }
    }

    /// Responds to an interaction with a loading state.
    pub async fn respond_loading(&self) -> Result<(), Error> {
        self.respond(
            format!(
                "{}/interactions/{}/{}/callback",
                BASE_URL, self.id, self.token
            ),
            InteractionResponse {
                type_: InteractionCallbackType::DeferredChannelMessageWithSource,
                data: None,
            },
        )
        .await
    }

    // Responds to an interaction with a message
    pub async fn respond_message(&self, msg: InteractionCallbackData) -> Result<(), Error> {
        self.respond(
            format!(
                "{}/interactions/{}/{}/callback",
                BASE_URL, self.id, self.token
            ),
            InteractionResponse {
                type_: InteractionCallbackType::ChannelMessageWithSource,
                data: Some(msg),
            },
        )
        .await
    }

    // Update the response that was sent with a new response
    pub async fn update_response(&self, response: InteractionResponse) -> Result<(), Error> {
        self.respond(
            format!("{}/interactions/{}/{}/m", BASE_URL, self.id, self.token),
            response,
        )
        .await
    }

    // TODO I think I have to move this into the individual thread because the requests have to be different for each one
    async fn respond(&self, uri: String, payload: InteractionResponse) -> Result<(), Error> {
        let route = RequestRoute {
            base_route: "interactions/<interaction_id>/<interaction_token>".to_string(),
            major_param: "".to_string(),
        };
        let request_builder = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let req = send_request_noparse(self.__ctx__.clone(), route, request_builder)
            .await;
        if let Err(e) = &req {
            println!("{:?}", e);
        }
        req
    }

    /// Gets an option from the interaction as type T. Panics if there is a data type mismatch.
    pub fn get_option<T>(&self, name: &str) -> Option<InteractionOption<T>>
    where InteractionOption<T>: From<InteractionDataOption> {
        self.data
        .as_ref()?.options
        .as_ref()?.iter().
        find(|o|o.name == name)
        .map(|o|o.to_owned().into())
    }
}
