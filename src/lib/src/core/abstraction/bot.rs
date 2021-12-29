use serde_json::json;

use crate::{
    core::{
        http::{rate_limit_client::RLClient, request_queue::BasicHttpQueue},
        interactions::handler::{websocket::WebsocketEventHandler, SocketClient},
        settings::Settings,
    },
    discord::resources::user::User,
    util::logger::print_debug,
    Registerable,
};

use super::{context::Context, event_dispatcher::EventDispatcher};

pub struct Bot {
    ctx: Context,
    event_dispatcher: EventDispatcher,
    token: String,
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
            settings: Settings::default(),
            cache: (),
        };
        let event_dispatcher = EventDispatcher::new();
        Self {
            ctx,
            event_dispatcher,
            token,
        }
    }

    pub fn settings(&mut self) -> &mut Settings {
        &mut self.ctx.settings
    }

    pub fn register_all(&mut self, to_register: Vec<&dyn Registerable>) {
        for event in to_register.iter() {
            event.register(&mut self.event_dispatcher);
        }
    }

    pub fn build(self) -> Bot {
        Bot::create_with_builder(self)
    }
}

impl Bot {
    pub fn builder(token: String) -> BotBuilder {
        BotBuilder::new(token)
    }

    pub fn create_with_builder(bldr: BotBuilder) -> Self {
        Bot {
            ctx: bldr.ctx,
            event_dispatcher: bldr.event_dispatcher,
            token: bldr.token,
        }
    }

    pub async fn listen(&self) {
        let event_handler = WebsocketEventHandler::create(self.ctx.clone()).await;

        print_debug("BOT", "Sending ".to_string());
        let cmd = json!({
            "op": 2,
            "d": {
                "token": self.token,
                "properties": {
                    "$os": "linux",
                    "$browser": "discord.rs",
                    "$device": "discord.rs",
                },
                "intents": 1 << 9,
            }
        });
        event_handler.send_command(cmd.to_string());
        print_debug("BOT", "Listening...".to_string());

        let cmds = event_handler.get_command_channel();
        while let Ok((command, data)) = cmds.recv() {
            self.event_dispatcher
                .route_event(self.ctx.clone(), command, data)
                .await;
        }
    }

    pub async fn get_user(&self) -> User {
        User::get(self.ctx.clone(), "@me".to_string())
            .await
            .unwrap()
    }
}
