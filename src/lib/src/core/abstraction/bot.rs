use std::borrow::Borrow;

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

use super::{
    context::Context, event_dispatcher::EventDispatcher, interaction_router::InteractionRouter,
};

pub struct Bot<'a> {
    ctx: Context,
    event_dispatcher: EventDispatcher<'a>,
    token: String,
    interaction_router: InteractionRouter,
}

impl<'a> Bot<'a> {
    pub fn new(token: String) -> Self {
        let client = RLClient::new(BasicHttpQueue::new(60));
        let ctx = Context {
            token: token.clone(),
            request_stream: client.get_req_sender(),
            settings: Settings::default(),
            cache: (),
        };
        let mut event_dispatcher = EventDispatcher::new();
        let mut interaction_router = InteractionRouter::new(ctx.clone());
        interaction_router.attatch(&mut event_dispatcher);
        Self {
            interaction_router,
            ctx,
            event_dispatcher,
            token,
        }
    }

    pub fn settings(&mut self) -> &mut Settings {
        &mut self.ctx.settings
    }

    pub fn register_all(&mut self, to_register: Vec<&dyn Registerable>) -> &mut Self {
        for event in to_register.iter() {
            event.register(
                self.ctx.clone(),
                &mut self.event_dispatcher,
                &mut self.interaction_router,
            );
        }
        self
    }

    pub async fn listen(&self) {
        let event_handler = WebsocketEventHandler::create(self.ctx.clone()).await;

        print_debug("BOT", "Identifying Self".to_string());
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

        let cmds = event_handler.get_command_channel();
        print_debug("BOT", "Listening...".to_string());
        while let Ok((command, data)) = cmds.recv() {
            self.event_dispatcher
                .route_event(self.ctx.clone(), command, data)
                .await;
        }
    }

    pub async fn get_user(&self) -> User {
        User::get_self(self.ctx.clone()).await.unwrap()
    }
}
