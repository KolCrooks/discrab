use discordrs_codegen::CommandArg;
use hyper::{Body, Method, Request};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        abstraction::{abstraction_traits::CommandArg, context::Context},
        http::rate_limit_client::{send_request, RequestRoute},
    },
    discord::{image_formats, snowflake::Snowflake},
    util::error::Error,
    BASE_URL,
};

use super::UserFlags;

/**
 * User Object
 * @docs https://discord.com/developers/docs/resources/user#user-object
 */
#[derive(Serialize, Deserialize, Clone, CommandArg)]
pub struct User {
    /**
     * The user's id
     * Example: `250726400149946368`
     * Required OAuth2 Scope: **identify**
     */
    pub id: Snowflake,

    /**
     * The user's username, not unique across the platform identify
     * Example: `"Kol"`
     * Required OAuth2 Scope: **identify**
     */
    pub username: String,

    /**
     * The user's 4-digit discord-tag
     * Example: `"9831"`
     * Required OAuth2 Scope: **identify**
     */
    pub discriminator: String,

    /**
     * The user's avatar hash
     * Example: `"6a1d232badfc2f317016cd54462cadb4"`
     * Required OAuth2 Scope: **identify**
     */
    pub avatar: Option<String>,

    /**
     * Whether the user belongs to an OAuth2 application
     * Example: true
     * Required OAuth2 Scope: **identify**
     */
    pub bot: Option<bool>,

    /**
     * whether the user is an Official Discord System user (part of the urgent message system)
     * Example: `false`
     * Required OAuth2 Scope: **identify**
     */
    pub system: Option<bool>,
    /**
     * Whether the user has two factor enabled on their account
     * Example: `false`
     * Required OAuth2 Scope: **identify**
     */
    pub mfa_enabled: Option<bool>,

    /**
     * The user's banner hash
     * Example: `false`
     * Required OAuth2 Scope: **identify**
     */
    pub banner: Option<String>,

    /**
     * The user's banner color encoded as an integer representation of hexadecimal color code
     * Example: `16711680`
     * Required OAuth2 Scope: **identify**
     */
    pub accent_color: Option<u32>,

    /**
     * The user's chosen language option
     * Example: `false`
     * Required OAuth2 Scope: **identify**
     */
    pub locale: Option<String>,

    /**
     * Whether the email on this account has been verified
     * Example: `false`
     * Required OAuth2 Scope: **email**
     */
    pub verified: Option<bool>,

    /**
     * The user's email
     * Example: "example@email.com"
     * Required OAuth2 Scope: **email**
     */
    pub email: Option<String>,

    /**
     * The flags on a user's account
     * Example: `64`
     * Required OAuth2 Scope: **identify**
     */
    pub flags: Option<UserFlags>,

    /**
     * The type of Nitro subscription on a user's account
     * Example: `0`
     * Required OAuth2 Scope: **identify**
     */
    pub premium_type: Option<u8>,

    /**
     * The public flags on a user's account
     * Example: `0`
     * Required OAuth2 Scope: **identify**
     */
    pub public_flags: Option<UserFlags>,
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
    pub fn get_avatar_url(
        &self,
        fmt: image_formats::Animated,
        size: Option<u32>,
    ) -> Option<String> {
        let size_str = match size {
            Some(s) => format!("?size={}", s),
            None => "".to_string(),
        };

        self.avatar.as_ref().map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}{}",
                self.id, avatar, fmt, size_str
            )
        })
    }

    /**
     * Gets the user's banner url
     *
     * @param fmt Image format of the banner
     *
     * @param size Size of the banner. If none specified, the largest size will be used.
     * The size must be a power of 2 between `16` and `4096`
     */
    pub fn get_banner_url(
        &self,
        fmt: image_formats::Animated,
        size: Option<u32>,
    ) -> Option<String> {
        let size_str = match size {
            Some(s) => format!("?size={}", s),
            None => "".to_string(),
        };

        self.banner.as_ref().map(|banner| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}{}",
                self.id, banner, fmt, size_str
            )
        })
    }

    pub async fn get(ctx: Context, id: String) -> Result<User, Error> {
        let route = RequestRoute {
            base_route: "/users".to_string(),
            major_param: "".to_string(),
        };
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(format!("{}/users/{}", BASE_URL, id))
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        send_request::<User>(ctx, route, request_builder).await
    }

    pub async fn get_self(ctx: Context) -> Result<User, Error> {
        User::get(ctx, "@me".to_string()).await
    }
}
