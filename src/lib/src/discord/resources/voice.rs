use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;

use super::guild::guild_member::GuildMember;
/**
 * Voice State
 * @docs https://discord.com/developers/docs/resources/voice#voice-state-object
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct VoiceState {
    /// the guild id this voice state is for
    pub guild_id: Option<Snowflake>,
    /// the channel id this user is connected to
    pub channel_id: Option<Snowflake>,
    /// the user id this voice state is for
    pub user_id: Snowflake,
    /// the guild member this voice state is for
    pub member: Option<GuildMember>,
    /// the session id for this voice state
    pub session_id: String,
    /// whether this user is deafened by the server
    pub deaf: bool,
    /// whether this user is muted by the server
    pub mute: bool,
    /// whether this user is locally deafened
    pub self_deaf: bool,
    /// whether this user is locally muted
    pub self_mute: bool,
    /// whether this user is streaming using "Go Live"
    pub self_stream: Option<bool>,
    /// whether this user's camera is enabled
    pub self_video: bool,
    /// whether this user is muted by the current user
    pub suppress: bool,
    /// the time at which the user requested to speak
    pub request_to_speak_timestamp: Option<String>,
}
