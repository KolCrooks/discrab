use super::typing::Interaction;
mod events;
pub mod gateway;
mod gateway_payload;
pub mod websocket;
trait InteractionHandler {
    fn get_incoming(&self) -> Vec<Interaction>;
}
