use serde::{Deserialize, Serialize};

use crate::{
    api::Snowflake,
    discord::resources::{application::Application, user::User},
};

/**
 * Integration Structure
 * @docs <https://discord.com/developers/docs/resources/guild#integration-object>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Integration {
    /// The id of the integration
    pub id: Snowflake,
    /// The name of the integration
    pub name: String,
    /// The type of the integration
    #[serde(rename = "type")]
    pub type_: IntegrationType,
    /// Is this integration enabled
    pub enabled: bool,
    /// Is this integration syncing
    pub syncing: bool,
    /// The id that this integration uses for "subscribers"
    pub role_id: Option<u64>,
    /// Whether emoticons should be synced for this integration (twitch only currently)
    pub enable_emoticons: Option<bool>,
    /// The behavior of expiring subscribers
    pub expire_behavior: Option<String>,
    /// The grace period (in days) before expiring subscribers
    pub expire_grace_period: Option<u64>,
    /// User for this integration
    pub user: Option<User>,
    /// Integration account information
    pub account: Account,
    /// When this integration was last synced
    pub synced_at: Option<String>,
    /// How many subscribers this integration has
    pub subscriber_count: Option<u64>,
    /// Has this integration been revoked
    pub revoked: Option<bool>,
    /// The bot/OAuth2 application for discord integrations
    pub application: Option<Application>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum IntegrationType {
    Twitch,
    Youtube,
    Discord,
}

/**
 * Integration Account
 * @docs <https://discord.com/developers/docs/resources/guild#integration-account-object>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    /// The id of the account
    pub id: String,
    /// The name of the account
    pub name: String,
}
