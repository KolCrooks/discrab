use super::snowflake::Snowflake;

pub struct User {
    /**
     * The user's id
     * Example: `250726400149946368`
     * Required OAuth2 Scope: **identify**
     */
    id:	Snowflake,

    /**
     * The user's username, not unique across the platform identify
     * Example: `"Kol"`
     * Required OAuth2 Scope: **identify**
     */
    username: String,

    /**
     * The user's 4-digit discord-tag
     * Example: `"9831"`
     * Required OAuth2 Scope: **identify**
     */
    discriminator: String,

    /**
     * The user's avatar hash
     * Example: `"6a1d232badfc2f317016cd54462cadb4"`
     * Required OAuth2 Scope: **identify**
     */
    avatar: Option<String>,

    /**
     * Whether the user belongs to an OAuth2 application
     * Example: true
     * Required OAuth2 Scope: **identify**
     */
    bot: Option<bool>,

    // system?	boolean	whether the user is an Official Discord System user (part of the urgent message system)	identify
    // mfa_enabled?	boolean	whether the user has two factor enabled on their account	identify
    // banner?	?string	the user's banner hash	identify
    // accent_color?	?integer	the user's banner color encoded as an integer representation of hexadecimal color code	identify
    // locale?	string	the user's chosen language option	identify
    // verified?	boolean	whether the email on this account has been verified	email
    // email?	?string	the user's email	email
    // flags?	integer	the flags on a user's account	identify
    // premium_type?	integer	the type of Nitro subscription on a user's account	identify
    // public_flags?	integer	the public flags on a user's account	identify
}

impl User {

    pub fn get_avatar_url(&self) -> String {
        format!("https://cdn.discordapp.com/avatars/{}/{}",
         &self.id.to_string(),
         self.avatar)
    }
}