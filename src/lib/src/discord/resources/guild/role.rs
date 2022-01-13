use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;

/**
 * Role Structure
 * @docs <https://discord.com/developers/docs/topics/permissions#role-object>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Role {
    /// role id
    pub id: Snowflake,
    /// role name
    pub name: String,
    /// integer representation of hexadecimal color code
    pub color: i64,
    /// if this role is pinned in the user listing
    pub hoist: bool,
    /// role icon hash
    pub icon: Option<String>,
    /// role icon hash
    pub icon_hash: Option<String>,
    /// position of this role
    pub position: i64,
    /// permission bit set
    pub permissions: Option<String>,
    /// whether this role is managed by an integration
    pub managed: bool,
    /// whether this role is mentionable
    pub mentionable: bool,
    /// the tags this role has
    pub tags: Option<RoleTags>,
}

/**
 * Role Tags Structure
 * @docs <https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct RoleTags {
    /// the id of the bot this role belongs to
    pub bot_id: Option<Snowflake>,
    /// the id of the integration this role belongs to
    pub integration_id: Option<Snowflake>,
    /// whether this is the guild's premium subscriber role
    pub premium_subscriber: Option<()>,
}
