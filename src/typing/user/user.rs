use super::super::{
    imageformats,
    snowflake::Snowflake,
};
 
use super::UserFlags;

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

    /**
     * whether the user is an Official Discord System user (part of the urgent message system)
     * Example: `false`
     * Required OAuth2 Scope: **identify**
     */
    system: Option<bool>,
    /**
     * Whether the user has two factor enabled on their account
     * Example: `false`
     * Required OAuth2 Scope: **identify**
     */
    mfa_enabled: Option<bool>,

    /**
     * The user's banner hash
     * Example: `false`
     * Required OAuth2 Scope: **identify**
     */
    banner: Option<String>,

    /**
     * The user's banner color encoded as an integer representation of hexadecimal color code
     * Example: `16711680`
     * Required OAuth2 Scope: **identify**
     */
    accent_color: Option<u32>,
    
    /**
     * The user's chosen language option
     * Example: `false`
     * Required OAuth2 Scope: **identify**
     */
    locale: Option<String>,
    
    /**
     * Whether the email on this account has been verified
     * Example: `false`
     * Required OAuth2 Scope: **email**
     */
    verified: Option<bool>,
    
    /**
     * The user's email
     * Example: "example@email.com"
     * Required OAuth2 Scope: **email**
     */
    email: Option<String>,

    /**
     * The flags on a user's account
     * Example: `64`
     * Required OAuth2 Scope: **identify**
     */
    flags: Option<UserFlags>,

    /**
     * The type of Nitro subscription on a user's account
     * Example: `0`
     * Required OAuth2 Scope: **identify**
     */
    premium_type: Option<u8>,

    /**
     * The public flags on a user's account
     * Example: `0`
     * Required OAuth2 Scope: **identify**
     */
    public_flags: Option<UserFlags>,
}

impl User {

    /**
     * Gets the user's avatar url
     * 
     * @param fmt Image format of the avatar
     * 
     * @param size Size of the avatar. If none specified, the largest size will be used.
     * The size must be a power of 2 between `16` and `4096` 
     */
    pub fn get_avatar_url(&self, fmt: imageformats::Animated, size: Option<u32>) -> Option<String> {
        let size_str = match size {
            Some(s) => format!("?size={}", s),
            None => "".to_string()
        };
          
        match self.avatar {
            Some(ref avatar) => 
                Some(format!("https://cdn.discordapp.com/avatars/{}/{}.{}{}",
                            self.id.to_string(),
                            avatar,
                            fmt.to_string(),
                            size_str)),
            None => None
        }
    }

    /**
     * Gets the user's banner url
     * 
     * @param fmt Image format of the banner
     * 
     * @param size Size of the banner. If none specified, the largest size will be used.
     * The size must be a power of 2 between `16` and `4096` 
     */
    pub fn get_banner_url(&self, fmt: imageformats::Animated, size: Option<u32>) -> Option<String> {
        let size_str = match size {
            Some(s) => format!("?size={}", s),
            None => "".to_string()
        };

        match self.banner {
            Some(ref banner) => 
            Some(format!("https://cdn.discordapp.com/avatars/{}/{}.{}{}",
                    self.id.to_string(),
                    banner,
                    fmt.to_string(),
                    size_str)),
            None => None
        }
    }

}