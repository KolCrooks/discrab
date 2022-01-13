## Creating a discord bot

#### Core interface:

Create a bot with:
```rust
use discrab::Bot;

let bot = Bot::new("TOKEN");
```

#### Creating an event listener:

This is an example of a message create event listener.
```rust
use discrab::{EventHandler, macros::event_handler, api::Channel, Events};

struct MsgEvent;

#[event_handler]
impl EventHandler<Message> for MsgEvent {
        /// This is going to be the event that the handler will listen for
        const EVENT_TYPE: Events = Events::MessageCreate;

        /// This function is called when the bot receives event with Self::EVENT_TYPE
        async fn handler(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!ping") {
            Channel::send_message(ctx.clone(), msg.channel_id.to_string(), "pong".to_string())
                .await
                .unwrap();
        }
    }
}
```
This was just an example of a MessageCreate event listener. You can listen for any event type within the `Events` enum.
