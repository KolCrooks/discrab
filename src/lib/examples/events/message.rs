use discrab::{macros::event_handler, EventHandler, events::Message, Events, Context};

pub struct MsgEvent;

#[event_handler]
impl EventHandler<Message> for MsgEvent {
    /// This is going to be the event that the handler will listen for
    const EVENT_TYPE: Events = Events::MessageCreate;

    /// This function is called when the bot receives event with Self::EVENT_TYPE
    async fn handler(&self, _: Context, msg: Message) {
        println!("[MESSAGE] {}", msg.content);
    }
}