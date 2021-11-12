use crate::discord::snowflake::Snowflake;
/**
 * Emoji Object
 * @docs https://discord.com/developers/docs/resources/emoji#emoji-object
 */
pub struct Emoji {
    /// Emoji id
    pub id: Snowflake,
    /// Emoji name
    pub name: Option<String>,
    /// Roles allowed to use this emoji
    pub roles: Vec<Snowflake>,
    /// User that created this emoji
    pub user: Option<Snowflake>,
    /// Whether this emoji must be wrapped in colons
    pub require_colons: bool,
    /// Whether this emoji is managed
    pub managed: bool,
    /// Whether this emoji is animated
    pub animated: bool,
    /// Whether this emoji can be used, may be false due to loss of Server Boosts
    pub available: bool,
}
