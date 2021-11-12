use super::typing::Interaction;
pub mod gateway;
pub mod websocket;

trait InteractionHandler {
    fn get_incoming(&self) -> Vec<Interaction>;
}
