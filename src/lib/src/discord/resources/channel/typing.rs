use crate::discord::snowflake::Snowflake;
use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * Channel Types
 * @docs https://discord.com/developers/docs/resources/channel#channel-object-channel-types
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ChannelType {
    /// A text channel within a server
    GuildText = 0,
    /// A direct message between users
    DM = 1,
    /// A voice channel within a server
    GuildVoice = 2,
    /// A direct message between multiple users
    GroupDm = 3,
    /// An organizational category that contains up to 50 channels
    GuildCategory = 4,
    /// A channel that users can follow and crosspost into their own server
    GuildNews = 5,
    /// A channel in which game developers can sell their game on Discord
    GuildStore = 6,
    /// A temporary sub-channel within a GUILD_NEWS channel
    GuildNewsThread = 10,
    /// A temporary sub-channel within a GUILD_TEXT channel
    GuildPublicThread = 11,
    /// A temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
    GuildPrivateThread = 12,
    /// A voice channel for hosting events with an audience
    GuildStageVoice = 13,
}

/**
 * Message Types
 * @docs https://discord.com/developers/docs/resources/channel#message-object-message-types
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]

pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionTier1 = 9,
    UserPremiumGuildSubscriptionTier2 = 10,
    UserPremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
}

/**
 * Message Activity Types
 * @docs https://discord.com/developers/docs/resources/channel#message-object-message-activity-types
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

/**
 * Message Activity Object
 * @docs https://discord.com/developers/docs/resources/channel#message-object-message-activity-structure
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageActivity {
    /// The type of message activity
    #[serde(rename = "type")]
    pub type_: MessageActivityType,
    /// The party_id from a Rich Presence event
    pub party_id: Option<String>,
}

/**
 * Message Reference Object
 * @docs https://discord.com/developers/docs/resources/channel#message-object-message-reference-structure
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageReference {
    /// The id of the originating message
    pub message_id: Option<String>,
    /// The id of the originating message's channel
    pub channel_id: Option<String>,
    /// The id of the originating message's guild
    pub guild_id: Option<String>,
    /// When sending, whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply) message, default true
    pub fail_if_not_exists: Option<bool>,
}

bitflags! {
    /// Message Flags
    /// @docs https://discord.com/developers/docs/resources/channel#message-object-message-flags
    #[derive(Serialize)]
    pub struct MessageFlags: u64 {
        const CROSSPOSTED = 1 << 0;
        const IS_CROSSPOST = 1 << 1;
        const SUPPRESS_EMBEDS = 1 << 2;
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        const URGENT = 1 << 4;
        const HAS_THREAD = 1 << 5;
        const EPHEMERAL = 1 << 6;
        const LOADING = 1 << 7;
    }
}

impl<'de> Deserialize<'de> for MessageFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;

        MessageFlags::from_bits(bits)
            .ok_or_else(|| serde::de::Error::custom(format!("Unexpected flags value {}", bits)))
    }
}

/**
 * Overwrite Object
 * See permissions for more information about the allow and deny fields.
 * @docs https://discord.com/developers/docs/resources/channel#overwrite-object-overwrite-structure
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct PermissionsOverwriteObject {
    /// The id of the role or user
    pub id: Snowflake,
    /// The type of the role or user
    #[serde(rename = "type")]
    pub type_: u8,
    /// The permissions that the role or user has
    pub allow: u64,
    /// The permissions that the role or user does not have
    pub deny: u64,
}

/**
* Thread Metadata Object
* The thread metadata object contains a number of thread-specific channel fields that are not needed by other channel types.
* @docs https://discord.com/developers/docs/resources/channel#message-object-thread-metadata-structure
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct ThreadMetadata {
    /// Whether the thread is archived
    pub archived: Option<bool>,
    /// Duration in minutes to automatically archive the thread after recent activity, can be set to: 60, 1440, 4320, 10080
    pub auto_archive_duration: Option<u64>,
    /// Timestamp when the thread's archive status was last changed, used for calculating recent activity
    pub archive_timestamp: Option<String>,
    /// Whether the thread is locked; when a thread is locked, only users with MANAGE_THREADS can unarchive it
    pub locked: Option<bool>,
    /// Whether non-moderators can add other non-moderators to a thread; only available on private threads
    pub invitable: Option<bool>,
}

/**
 * Thread Member Object
 * A thread member is used to indicate whether a user has joined a thread or not.
* @docs https://discord.com/developers/docs/resources/channel#message-object-thread-member-structure
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct ThreadMember {
    /// The id of the thread
    pub id: Snowflake,
    /// The id of the user
    pub user_id: Snowflake,
    /// The time the current user last joined the thread
    pub join_timestamp: String,
    /// Any user-thread settings, currently only used for notifications
    pub flags: u64,
}

/**
 * Video Quality
 * @docs https://discord.com/developers/docs/resources/channel#channel-object-video-quality-modes
 */
#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum VideoQualityMode {
    /// Discord chooses the quality for optimal performance
    Auto = 1,
    /// 720p
    Full = 2,
}
