use super::typing::Interaction;

trait InteractionHandler {
    fn get_incoming() -> Vec<Interaction>;
}
