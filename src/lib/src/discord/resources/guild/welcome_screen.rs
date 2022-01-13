use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;

/**
 * Welcome Screen Object
 * @docs <https://discord.com/developers/docs/resources/guild#welcome-screen-object>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct WelcomeScreen {
    /// the server description shown in the welcome screen
    pub description: Option<String>,
    /// the channels shown in the welcome screen, up to 5
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

/**
 * Welcome Screen Channel Structure
 * @docs <https://discord.com/developers/docs/resources/guild#welcome-screen-object-welcome-screen-channel-structure>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct WelcomeScreenChannel {
    /// the channel's id
    pub channel_id: Snowflake,
    /// the description shown for the channel
    pub description: Option<String>,
    /// the emoji id, if the emoji is custom
    pub emoji_id: Option<Snowflake>,
    /// the emoji name if custom, the unicode character if standard, or null if no emoji is set
    pub emoji_name: Option<String>,
}
