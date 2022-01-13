use serde::{Deserialize, Serialize};

use crate::discord::resources::emoji::Emoji;

/**
 * Reaction Object
 * @docs <https://discord.com/developers/docs/resources/channel#reaction-object>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Reaction {
    /// times this emoji has been used to react
    pub count: i64,
    /// whether the current user reacted using this emoji
    pub me: bool,
    /// emoji information
    pub emoji: Emoji,
}
