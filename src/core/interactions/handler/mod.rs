use super::typing::Interaction;
mod gateway;
mod websocket;

trait InteractionHandler {
    fn get_incoming(&self) -> Vec<Interaction>;
}
