use crate::discord::{snowflake::Snowflake, teams::Team};

use bitfield::bitfield;

use super::user::User;

/**
 * Represents a Discord application.
 * https://discord.com/developers/docs/resources/application#application-object-application-structure
 */
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

bitfield! {
    /// Application Flags
    pub struct ApplicationFlags(u64);

    u8;
    pub GATEWAY_PRESENCE, _: 0, 1;
    pub GATEWAY_PRESENCE_LIMITED, _: 1, 2;
    pub GATEWAY_GUILD_MEMBERS, _: 2, 3;
    pub GATEWAY_GUILD_MEMBERS_LIMITED, _: 3, 4;
    pub VERIFICATION_PENDING_GUILD_LIMIT, _: 4, 5;
    pub EMBEDDED, _: 5, 6;
    pub GATEWAY_MESSAGE_CONTENT, _: 6, 7;
    pub GATEWAY_MESSAGE_CONTENT_LIMITED, _: 7, 8;
}
