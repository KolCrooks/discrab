use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::discord::gateway::presence::PresenceUpdateEvent;

#[derive(Serialize, Deserialize)]
/**
 * Used to trigger the initial handshake with the gateway.
 * @docs https://discord.com/developers/docs/topics/gateway#identify-identify-structure
*/
pub struct IdentifyPayloadData {
    pub token: String,
    pub properties: Option<IdentifyProperties>,
    pub compress: Option<bool>,
    pub large_threshold: Option<u64>,
    pub shard: Option<[u64; 2]>,
    pub presence: Option<PresenceUpdateEvent>,
    pub intents: Intents,
}

impl IdentifyPayloadData {
    pub fn new(token: String) -> Self {
        Self {
            token,
            properties: None,
            compress: None,
            large_threshold: None,
            shard: None,
            presence: None,
            intents: Intents::default(),
        }
    }
}

/**
 * Identify Connection Properties
 */
#[derive(Serialize, Deserialize)]
pub struct IdentifyProperties {
    /// your operating system
    #[serde(rename = "$os")]
    pub os: String,
    /// your library name
    #[serde(rename = "$browser")]
    pub browser: String,
    /// your library name
    #[serde(rename = "$device")]
    pub device: String,
}

bitflags! {
    #[derive(Serialize)]
    pub struct Intents: u64 {
        const GUILDS = 1 << 0;
        const GUILD_MEMBERS = 1 << 1;
        const GUILD_BANS = 1 << 2;
        const GUILD_EMOJIS_AND_STICKERS = 1 << 3;
        const GUILD_INTEGRATIONS = 1 << 4;
        const GUILD_WEBHOOKS = 1 << 5;
        const GUILD_INVITES = 1 << 6;
        const GUILD_VOICE_STATES = 1 << 7;
        const GUILD_PRESENCES = 1 << 8;
        const GUILD_MESSAGES = 1 << 9;
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        const GUILD_MESSAGE_TYPING = 1 << 11;
        const DIRECT_MESSAGES = 1 << 12;
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        const DIRECT_MESSAGE_TYPING = 1 << 14;
        const GUILD_SCHEDULED_EVENTS = 1 << 16;
    }
}

impl<'de> Deserialize<'de> for Intents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;

        Intents::from_bits(bits).ok_or(serde::de::Error::custom(format!(
            "Unexpected flags value {}",
            bits
        )))
    }
}

impl Default for Intents {
    fn default() -> Self {
        Self::GUILDS
            | Self::GUILD_MEMBERS
            | Self::GUILD_BANS
            | Self::GUILD_EMOJIS_AND_STICKERS
            | Self::GUILD_INTEGRATIONS
            | Self::GUILD_WEBHOOKS
            | Self::GUILD_INVITES
            | Self::GUILD_VOICE_STATES
            | Self::GUILD_PRESENCES
            | Self::GUILD_MESSAGES
            | Self::GUILD_MESSAGE_REACTIONS
            | Self::GUILD_MESSAGE_TYPING
            | Self::DIRECT_MESSAGES
            | Self::DIRECT_MESSAGE_REACTIONS
            | Self::DIRECT_MESSAGE_TYPING
            | Self::GUILD_SCHEDULED_EVENTS
    }
}
