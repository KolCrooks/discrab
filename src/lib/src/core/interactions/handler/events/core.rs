use serde::{Deserialize, Serialize};

use crate::core::interactions::handler::gateway_payload::PayloadOpcode;

use super::PayloadData;

#[derive(Serialize, Deserialize)]
/**
   Sent on connection to the websocket. Defines the heartbeat interval that the client should heartbeat to.
   @docs https://discord.com/developers/docs/topics/gateway#hello
*/
pub struct HelloPayloadData {
    /// the interval (in milliseconds) the client should heartbeat with
    pub heartbeat_interval: u64,
}
impl PayloadData for HelloPayloadData {
    fn get_opcode(&self) -> PayloadOpcode {
        PayloadOpcode::Heartbeat
    }
}

pub type HeartBeatPayloadData = Option<u64>;

impl PayloadData for HeartBeatPayloadData {
    fn get_opcode(&self) -> PayloadOpcode {
        PayloadOpcode::Heartbeat
    }
}
