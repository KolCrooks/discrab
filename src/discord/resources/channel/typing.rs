use crate::discord::snowflake::Snowflake;
use bitfield::bitfield;

/**
 * Channel Types
 * @docs https://discord.com/developers/docs/resources/channel#channel-object-channel-types
 */
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
pub struct MessageActivity {
    /// The type of message activity
    pub type_: MessageActivityType,
    /// The party_id from a Rich Presence event
    pub party_id: Option<String>,
}

/**
 * Message Reference Object
 * @docs https://discord.com/developers/docs/resources/channel#message-object-message-reference-structure
 */
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

bitfield! {
    pub struct MessageFlags(u64);

    u8;
    /// this message has been published to subscribed channels (via Channel Following)
    pub crossposted, _: 0, 1;
    /// this message originated from a message in another channel (via Channel Following)
    pub is_crosspost, _: 1, 2;
    /// do not include any embeds when serializing this message
    pub suppress_embeds, _: 2, 3;
    /// the source message for this crosspost has been deleted (via Channel Following)
    pub source_message_deleted, _: 3, 4;
    /// this message came from the urgent message system
    pub urgent, _: 4, 5;
    /// this message has an associated thread, with the same id as the message
    pub has_thread, _: 5, 6;
    /// this message is only visible to the user who invoked the Interaction
    pub ephemeral, _: 6, 7;
    /// this message is an Interaction Response and the bot is "thinking"
    pub loading, _: 7, 8;
}

/**
 * Overwrite Object
 * See permissions for more information about the allow and deny fields.
 * @docs https://discord.com/developers/docs/resources/channel#overwrite-object-overwrite-structure
 */
pub struct PermissionsOverwriteObject {
    /// The id of the role or user
    pub id: Snowflake,
    /// The type of the role or user
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

pub enum VideoQualityMode {
    /// Discord chooses the quality for optimal performance
    Auto = 1,
    /// 720p
    Full = 2,
}
