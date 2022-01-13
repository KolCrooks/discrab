use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::snowflake::Snowflake;

use super::user::User;
/**
 * Sticker Object
 * @docs <https://discord.com/developers/docs/resources/sticker#sticker-item-object>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Sticker {
    /// id of the sticker
    pub id: Snowflake,
    /// for standard stickers, id of the pack the sticker is from
    pub pack_id: Option<Snowflake>,
    /// name of the sticker
    pub name: String,
    /// description of the sticker
    pub description: Option<String>,
    /// autocomplete/suggestion tags for the sticker (max 200 characters)
    pub tags: String,
    /**
     * @deprecated
     * previously the sticker asset hash, now an empty string
     */
    pub asset: String,
    /// type of sticker
    #[serde(rename = "type")]
    pub type_: StickerType,
    /// type of sticker format
    pub format_type: StickerFormatType,
    /// whether this guild sticker can be used, may be false due to loss of Server Boosts
    pub available: Option<bool>,
    /// id of the guild that owns this sticker
    pub guild_id: Option<Snowflake>,
    /// the user that uploaded the guild sticker
    pub user: Option<User>,
    /// the standard sticker's sort order within its pack
    pub sort_value: Option<i32>,
}

/**
 * Sticker Item Object
 * The smallest amount of data required to render a sticker. A partial sticker object.
 * @docs <https://discord.com/developers/docs/resources/sticker#sticker-item-object>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct StickerItem {
    /// id of the sticker
    pub id: Snowflake,
    /// name of the sticker
    pub name: String,
    /// type of sticker format
    pub format_type: i64,
}

/**
 * Sticker Types
 * @docs <https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-types>
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum StickerType {
    /// an official sticker in a pack, part of Nitro or in a removed purchasable pack
    Standard = 1,
    /// a sticker uploaded to a Boosted guild for the guild's members
    Guild = 2,
}

/**
 * Sticker Format Types
 * @docs <https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-format-types>
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum StickerFormatType {
    PNG = 1,
    APNG = 2,
    Lottie = 3,
}
