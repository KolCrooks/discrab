use std::thread;

use async_std::task::block_on;
use serde_json::json;

use crate::{
    core::{
        http::{rate_limit_client::RLClient, request_queue::BasicHttpQueue},
        interactions::handler::{websocket::WebsocketEventHandler, SocketClient},
    },
    discord::resources::{channel::message::Message, user::User},
};

use super::{
    context::Context,
    event_dispatcher::{self, EventDispatcher},
};

pub struct Bot {
    ctx: Context,
}

pub struct BotBuilder {
    ctx: Context,
    pub event_dispatcher: EventDispatcher,
    token: String,
}

impl BotBuilder {
    pub fn new(token: String) -> Self {
        let client = RLClient::new(BasicHttpQueue::new(60));
        let ctx = Context {
            token: token.clone(),
            request_stream: client.get_req_sender(),
            cache: (),
        };
        let mut event_dispatcher = EventDispatcher::new();

        Self {
            ctx,
            event_dispatcher,
            token,
        }
    }

    pub async fn build(self) -> Bot {
        Bot::create(self).await
    }
}

impl Bot {
    pub async fn create(bldr: BotBuilder) -> Self {
        let ctx = bldr.ctx;
        let bot = Bot { ctx: ctx.clone() };

        let ctx_cp = ctx;
        thread::spawn(move || {
            block_on(async {
                let ctx = ctx_cp;

                let event_handler = WebsocketEventHandler::create(ctx.clone()).await;
                let event_dispatcher = bldr.event_dispatcher;
                let cmd = json!({
                    "op": 2,
                    "d": {
                        "token": bldr.token,
                        "properties": {
                            "$os": "linux",
                            "$browser": "discord.rs",
                            "$device": "discord.rs",
                        },
                        "intents": 1 << 9,
                    }
                });
                event_handler.send_command(cmd.to_string());

                let cmds = event_handler.get_command_channel();
                while let Ok((command, data)) = cmds.recv() {
                    block_on((&event_dispatcher).route_event(ctx.clone(), command, data));
                }
            })
        });

        bot
    }

    pub async fn get_user(&self) -> User {
        User::get(self.ctx.clone(), "@me".to_string())
            .await
            .unwrap()
    }

    pub fn listen(&self) {
        // TODO
    }
}
