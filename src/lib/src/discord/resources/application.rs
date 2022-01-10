use crate::{
    core::http::rate_limit_client::{send_request, RequestRoute},
    discord::{snowflake::Snowflake, teams::Team},
    util::error::Error,
    Context, BASE_URL,
};

use bitflags::bitflags;
use hyper::{Body, Method, Request};
use serde::{Deserialize, Deserializer, Serialize};

use super::user::User;

/**
 * Represents a Discord application.
 * https://discord.com/developers/docs/resources/application#application-object-application-structure
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Application {
    /// The id of the app
    pub id: Snowflake,
    /// The name of the app
    pub name: String,
    /// The icon hash of the app
    pub icon: Option<String>,
    /// The description of the app
    pub description: String,
    /// An array of rpc origin urls, if rpc is enabled
    pub rpc_origins: Option<Vec<String>>,
    /// When false only app owner can join the app's bot to guilds
    pub bot_public: bool,
    /// When true the app's bot will only join upon completion of the full oauth2 code grant flow
    pub bot_require_code_grant: bool,
    /// The url of the app's terms of service
    pub terms_of_service_url: Option<String>,
    /// The url of the app's privacy policy
    pub privacy_policy_url: Option<String>,
    /// Partial user object containing info on the owner of the application
    pub owner: Option<User>,
    /// If this application is a game sold on Discord, this field will be the summary field for the store page of its primary sku
    pub summary: String,
    /// The hex encoded key for verification in interactions and the GameSDK's GetTicket
    pub verify_key: String,
    /// If the application is a game sold on Discord, this field will be the guild to which it has been linked
    pub team: Option<Team>,
    /// If this application is a game sold on Discord, this field will be the id of the "Game SKU" that is created, if exists
    pub guild_id: Option<Snowflake>,
    /// If this application is a game sold on Discord, this field will be the URL slug that links to the store page
    pub primary_sku_id: Option<Snowflake>,
    /// If this application is a game sold on Discord, this field will be the URL slug that links to the store page
    pub slug: Option<String>,
    /// The application's default rich presence invite cover image hash
    pub cover_image: Option<String>,
    /// The application's public flags
    pub flags: Option<ApplicationFlags>,
}

bitflags! {
    /// Application Flags
    /// @docs https://discord.com/developers/docs/resources/application#application-object-application-flags
    #[derive(Serialize)]
    pub struct ApplicationFlags: u64 {
        const GATEWAY_PRESENCE = 1 << 12;
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        const EMBEDDED = 1 << 17;
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
    }
}

impl<'de> Deserialize<'de> for ApplicationFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;

        ApplicationFlags::from_bits(bits)
            .ok_or_else(|| serde::de::Error::custom(format!("Unexpected flags value {}", bits)))
    }
}

impl Application {
    /// Gets the application associated with the bot
    pub async fn get_self(ctx: Context) -> Result<Application, Error> {
        let route = RequestRoute {
            base_route: "/oauth2/applications".to_string(),
            major_param: "".to_string(),
        };
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(format!("{}/oauth2/applications/@me", BASE_URL))
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        send_request(ctx, route, request_builder).await
    }
}
