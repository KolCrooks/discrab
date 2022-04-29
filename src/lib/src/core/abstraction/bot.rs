use std::sync::Arc;

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
    context::Context, event_dispatcher::EventDispatcher, interaction_router::InteractionRouter, traits::{RegisterableType, RegFns},
};

/// The main bot abstraction
///
/// ```rust,no_run
///  // Create a new bot instance with a token
///  let bot = Bot::new(token);
///  // Register Commands/Events that you want to listen for
///  register_commands!(bot, command, ...);
///  // Have the bot listen for events and commands, and await the bot to finish listening (Shouldn't escape unless there is an error)
///  bot.listen().await;
/// ```
pub struct Bot {
    /// Global context for the bot
    ctx: Context,
    /// The event dispatcher that distributes events to the registered handlers
    event_dispatcher: EventDispatcher,
    /// The token associated with the bot
    token: String,
    /// Interaction router that distributes interactions to the respective handlers. Is registered with the event dispatcher
    interaction_router: Arc<InteractionRouter>,
}

impl Bot {
    /// Create a new bot instance with a token. Your bot's token can be found in the discord developer portal
    pub fn new(token: String) -> Self {
        let client = RLClient::new(BasicHttpQueue::new(60));
        let ctx = Context {
            token: token.clone(),
            request_stream: client.get_req_sender(),
            settings: Settings::default(),
            cache: (),
        };
        let event_dispatcher = EventDispatcher::new();
        let interaction_router = Arc::new(InteractionRouter::new());

        Self {
            interaction_router,
            ctx,
            event_dispatcher,
            token,
        }
    }

    /// Get the settings associated with the bot's context
    pub fn settings(&mut self) -> &mut Settings {
        &mut self.ctx.settings
    }

    /// You can use this to register a command handler, or an interaction handler. The Registerable Trait is implemented for you through the `#[event_handler]` or `#[command]` macro/
    pub async fn register(mut self, to_register: Arc<impl Registerable + RegFns>) -> Self {
        let registerable_type= to_register.get_reg_type();
            match registerable_type {
                RegisterableType::Event => {
                    to_register.reg_event(&mut self.event_dispatcher);
                }
                RegisterableType::Command => {
                    to_register.reg_command(self.ctx.clone(), self.interaction_router.clone())
                }
                _=> ()
            }
        self
    }

    /// Listen for events and commands. This will block the thread until the bot is closed (when awaited).
    pub async fn listen(&mut self) {
        let event_handler = WebsocketEventHandler::create(self.ctx.clone()).await;

        // Register the interaction router
        self.event_dispatcher
            .InteractionCtx
            .subscribe(self.interaction_router.clone());

        if self.ctx.settings.debug {
            print_debug("BOT", "Identifying Self".to_string());
        }

        // Identify object for the bot
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

        // Send the identify object to the websocket
        event_handler.send_command(cmd.to_string());

        // Listen for events
        let cmds = event_handler.get_command_channel();
        if self.ctx.settings.debug {
            print_debug("BOT", "Listening...".to_string());
        }
        
        while let Ok((command, data)) = cmds.recv() {
            self.event_dispatcher
                .route_event(self.ctx.clone(), command, data);
        }
    }

    /// Get the discord user associated with the bot
    pub async fn get_user(&self) -> User {
        User::get_self(self.ctx.clone()).await.unwrap()
    }
}
