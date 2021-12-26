use super::typing::Interaction;
mod events;
pub mod gateway;
mod gateway_payload;
pub mod websocket;
pub trait InteractionHandler {
    fn get_incoming(&self) -> Vec<Interaction>;
    fn send_command(&self, command: String);
}
