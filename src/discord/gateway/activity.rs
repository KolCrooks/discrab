use crate::discord::{
    resources::{emoji::Emoji, user::User},
    snowflake::Snowflake,
};

/**
 * Activity Object
 * @docs https://discord.com/developers/docs/topics/gateway#activity-object
 */
pub struct Activity {
    /// The activity's name
    pub name: String,
    /// activity type
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
    pub assets: Option<Assets>,
    /// secrets for Rich Presence joining and spectating
    pub secrets: Option<Secrets>,
    /// whether or not the activity is an instanced game session
    pub instance: Option<bool>,
    /// activity flags ORd together, describes what the payload includes
    pub flags: Option<i32>,
    /// the custom buttons shown in the Rich Presence (max 2)
    pub buttons: Option<Vec<Button>>,
}

/**
 * Activity Type
 * @docs https://discord.com/developers/docs/topics/gateway#activity-object-activity-types
 */
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
 * @docs https://discord.com/developers/docs/topics/gateway#activity-object-party-object
 */
pub struct ActivityParty {
    /// the id of the party
    pub id: Option<String>,
    /// used to show the party's current and maximum size
    pub size: Option<(i32, i32)>,
}

/**
 * Timestamps for when the activity started and ended
 * @docs https://discord.com/developers/docs/topics/gateway#activity-object-activity-timestamps
 */
pub struct ActivityTimestamps {
    /// unix timestamp (in milliseconds) of when the activity started
    pub start: Option<i64>,
    /// unix timestamp (in milliseconds) of when the activity ends
    pub end: Option<i64>,
}
