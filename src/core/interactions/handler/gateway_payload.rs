use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize)]
pub struct Payload<T> {
    #[serde(rename = "op")]
    pub op_code: PayloadOpcode,
    #[serde(rename = "d")]
    pub data: T,
    #[serde(rename = "s")]
    pub sequence_num: Option<u32>,
    #[serde(rename = "t")]
    pub event_name: Option<String>,
}

/**
 * Gateway Opcodes
 * @docs https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
 */
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum PayloadOpcode {
    /**
     * An event was dispatched.
     * Client Action: Receive
     */
    Dispatch = 0,
    /**
     * Fired periodically by the client to keep the connection alive.
     * Client Action: Send/Receive
     */
    Heartbeat = 1,
    /**
     * Starts a new session during the initial handshake.
     * Client Action: Send
     */
    Identify = 2,
    /**
     * Update the client's presence.
     * Client Action: Send
     */
    PresenceUpdate = 3,
    /**
     * Used to join/leave or move between voice channels.
     * Client Action: Send
     */
    VoiceStateUpdate = 4,
    /**
     * Resume a previous session that was disconnected.
     * Client Action: Send
     */
    Resume = 6,
    /**
     * You should attempt to reconnect and resume immediately.
     * Client Action: Receive
     */
    Reconnect = 7,
    /**
     * Request information about offline guild members in a large guild.
     * Client Action: Send
     */
    RequestGuildMembers = 8,
    /**
     * The session has been invalidated. You should reconnect and identify/resume accordingly.
     * Client Action: Receive
     */
    InvalidSession = 9,
    /**
     * Sent immediately after connecting, contains the heartbeat_interval to use.
     * Client Action: Receive
     */
    Hello = 10,
    /**
     * Sent in response to receiving a heartbeat to acknowledge that it has been received.
     * Client Action: Receive
     */
    HeartbeatAck = 11,
}

#[derive(Serialize, Deserialize)]
pub struct HelloPayloadData {
    pub heartbeat_interval: u64,

    pub _trace: Vec<String>,
}