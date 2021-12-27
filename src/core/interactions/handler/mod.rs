use crossbeam_channel::Receiver;
use serde_json::Value;

pub mod events;
pub mod gateway;
mod gateway_payload;
pub mod websocket;
pub trait SocketClient {
    fn get_command_channel(&self) -> Receiver<(String, Value)>;
    fn send_command(&self, command: String);
}
