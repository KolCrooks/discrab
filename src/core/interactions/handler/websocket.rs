use std::thread;

use crate::core::{
    http::rate_limit_client::{RLClient, RequestRoute},
    interactions::typing::Interaction,
};

use super::{
    gateway::{get_gateway, Gateway},
    InteractionHandler,
};
use async_std::task::block_on;
use crossbeam_channel::{unbounded, Receiver, Sender};
use hyper::{body::Body, Request};
use simd_json;
use tungstenite::{connect, Message, WebSocket};

pub struct WebsocketInteractionHandler {
    receiver: Receiver<Interaction>,
}

impl WebsocketInteractionHandler {
    pub fn new(http_client: &RLClient) -> WebsocketInteractionHandler {
        let (s, r) = unbounded();
        let handler = WebsocketInteractionHandler { receiver: r };

        block_on(async {
            // TODO so the gateway says that it shouldn't be cached. WHAT DOES THIS MEAN????
            // does it mean not cached between instances, and having it get a new gateway on startup?
            // or does it want use to periodically get a new gateway while the bot is running? plz help
            let gateway = get_gateway(http_client).await.unwrap();
            thread::Builder::new()
                .name("Websocket_Interaction_Handler".to_string())
                .spawn(move || block_on(WebsocketInteractionHandler::run(s, gateway)))
                .unwrap();
        });
        handler
    }

    async fn run(sender: Sender<Interaction>, gateway: Gateway) {
        println!("Connecting to {}", gateway.url);
        let (mut socket, response) = connect(gateway.url).unwrap();
        // loop {
        //     let mut x = socket.read_message().unwrap().into_data();
        //     let json = simd_json::from_slice(&mut x);
        // }
    }
}

impl InteractionHandler for WebsocketInteractionHandler {
    fn get_incoming(&self) -> Vec<Interaction> {
        self.receiver.try_iter().collect()
    }
}
