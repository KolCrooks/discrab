use super::super::snowflake::Snowflake;

/**
 * Represents a Discord application.
 * https://discord.com/developers/docs/resources/application#application-object-application-structure
 */
pub struct Application {
    /**
     * The Id of the App.
     */
    id: Snowflake,
    /**
     * The name of the App.
     */
    name: String,
    /**
     * 	the icon hash of the app
     */
    icon: Option<String>,
    /**
     * the description of the app
     */
    description: String,
    /**
     * An array of rpc origin urls, if rpc is enabled
     */
    // TODO (Kol): Should this be a vec?
    rpc_origins: Option<Vec<String>>,
    /**
     * When false only app owner can join the app's bot to guilds
     */
    bot_public: bool,
    bot_require_code_grant: bool,
    terms_of_service_url: Option<String>,
    private_policy_url: Option<String>,
    // owner: 
}
