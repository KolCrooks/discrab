use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::{resources::emoji::Emoji, snowflake::Snowflake};

/**
 * Activity Object
 * @docs <https://discord.com/developers/docs/topics/gateway#activity-object>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Activity {
    /// The activity's name
    pub name: String,
    /// activity type
    #[serde(rename = "type")]
    pub type_: ActivityType,
    /// stream url, is validated when type is 1
    pub url: Option<String>,
    /// unix timestamp (in milliseconds) of when the activity was added to the user's session
    pub created_at: i64,
    /// unix timestamps for start and/or end of the game
    pub timestamps: Option<ActivityTimestamps>,
    /// application id for the game
    pub application_id: Option<Snowflake>,
    /// what the player is currently doing
    pub details: Option<String>,
    /// the user's current party status
    pub state: Option<String>,
    /// emoji used for a custom status
    pub emoji: Option<Emoji>,
    /// information for the current party of the player
    pub party: Option<ActivityParty>,
    /// images for the presence and their hover texts
    pub assets: Option<ActivityAssets>,
    /// secrets for Rich Presence joining and spectating
    pub secrets: Option<ActivitySecrets>,
    /// whether or not the activity is an instanced game session
    pub instance: Option<bool>,
    /// activity flags ORd together, describes what the payload includes
    pub flags: Option<ActivityFlags>,
    /// the custom buttons shown in the Rich Presence (max 2)
    pub buttons: Option<Vec<ActivityButton>>,
}

/**
 * Activity Type
 * @docs <https://discord.com/developers/docs/topics/gateway#activity-object-activity-types>
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ActivityType {
    /// Playing {name}
    /// Example: "Playing Rocket League"
    Game = 0,
    /// Streaming {details}
    /// Example: "Streaming Rocket League"
    Streaming = 1,
    /// Listening to {name}
    /// Example: "Listening to Spotify"
    Listening = 2,
    /// Watching {name}
    /// Example: "Watching YouTube Together"
    Watching = 3,
    /// Custom {emoji} {name}
    /// Example: ":smiley: I am cool"
    Custom = 4,
    /// Competing in {name}
    /// Example: "Competing in Arena World Champions"
    Competing = 5,
}

/**
 * Activity Party
 * @docs <https://discord.com/developers/docs/topics/gateway#activity-object-party-object>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ActivityParty {
    /// the id of the party
    pub id: Option<String>,
    /// used to show the party's current and maximum size
    pub size: Option<(i32, i32)>,
}

/**
 * Timestamps for when the activity started and ended
 * @docs <https://discord.com/developers/docs/topics/gateway#activity-object-activity-timestamps>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ActivityTimestamps {
    /// unix timestamp (in milliseconds) of when the activity started
    pub start: Option<i64>,
    /// unix timestamp (in milliseconds) of when the activity ends
    pub end: Option<i64>,
}

/**
* Activity Assets
* @docs <https://discord.com/developers/docs/topics/gateway#activity-object-activity-assets>
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct ActivityAssets {
    /// the id for a large asset of the activity, usually a snowflake
    pub large_image: Option<String>,
    /// text displayed when hovering over the large image of the activity
    pub large_text: Option<String>,
    /// the id for a small asset of the activity, usually a snowflake
    pub small_image: Option<String>,
    /// text displayed when hovering over the small image of the activity
    pub small_text: Option<String>,
}

/**
 * Activity Secrets
 * @docs <https://discord.com/developers/docs/topics/gateway#activity-object-activity-secrets>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ActivitySecrets {
    /// the secret for joining a party
    pub join: Option<String>,
    /// the secret for spectating a game
    pub spectate: Option<String>,
    /// the secret for a specific instanced match
    #[serde(rename = "match")]
    pub match_: Option<String>,
}

bitflags! {
    /**
     * Activity Flags
     * @docs <https://discord.com/developers/docs/topics/gateway#activity-object-activity-flags>
     */
    #[derive(Serialize)]
    pub struct ActivityFlags: u64 {
        const INSTANCE = 1 << 0;
        const JOIN = 1 << 1;
        const SPECTATE = 1 << 2;
        const JOIN_REQUEST = 1 << 3;
        const SYNC = 1 << 4;
        const PLAY = 1 << 5;
        const PARTY_PRIVACY_FRIENDS = 1 << 6;
        const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;
        const EMBEDDED = 1 << 8;
    }
}

impl<'de> Deserialize<'de> for ActivityFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;

        ActivityFlags::from_bits(bits)
            .ok_or_else(|| serde::de::Error::custom(format!("Unexpected flags value {}", bits)))
    }
}

/**
 * Activity Buttons
 * When received over the gateway, the buttons field is an array of strings, which are the button labels. Bots cannot access a user's activity button URLs. When sending, the buttons field must be an array of the below object:
 * @docs <https://discord.com/developers/docs/topics/gateway#activity-object-activity-buttons>
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ActivityButton {
    /// the text shown on the button (1-32 characters)
    pub label: String,
    /// the url opened when clicking the button (1-512 characters)
    pub url: String,
}
