use std::thread;

use crate::core::{
    http::rate_limit_client::RLClient,
    interactions::{
        handler::gateway_payload::{HelloPayloadData, Payload},
        typing::Interaction,
    },
};

use super::{
    gateway::{get_gateway, Gateway},
    InteractionHandler,
};
use async_std::task::block_on;
use crossbeam_channel::{unbounded, Receiver, Sender};

// use simd_json;
use serde_json::Value;

use tungstenite::connect;

pub struct WebsocketInteractionHandler {
    receiver: Receiver<Interaction>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl WebsocketInteractionHandler {
    pub fn new(http_client: &RLClient) -> WebsocketInteractionHandler {
        let (s, r) = unbounded();
        let mut handler = WebsocketInteractionHandler {
            receiver: r,
            thread_handle: None,
        };

        handler.thread_handle = Some(block_on(async {
            // TODO so the gateway says that it shouldn't be cached. WHAT DOES THIS MEAN????
            // does it mean not cached between instances, and having it get a new gateway on startup?
            // or does it want use to periodically get a new gateway while the bot is running? plz help
            let gateway = get_gateway(http_client).await.unwrap();
            thread::Builder::new()
                .name("Websocket_Interaction_Handler".to_string())
                .spawn(move || block_on(WebsocketInteractionHandler::run(s, gateway)))
                .unwrap()
        }));
        handler
    }

    async fn run(sender: Sender<Interaction>, gateway: Gateway) {
        let url = url::Url::parse(&format!("{}/?v=9&encoding=json", gateway.url)).unwrap();
        // println!("Connecting to {}", url);

        let (mut socket, response) = connect(url).expect("Can't connect");
        // println!("{}", response.status());

        let mut hello_msg = socket.read_message().unwrap().into_data();

        let hello_payload: Payload<HelloPayloadData> =
            simd_json::from_slice(&mut *hello_msg).expect("PLEASE WORK");
        let heartbeat_interval = hello_payload.data.heartbeat_interval;

        loop {
            thread::sleep(std::time::Duration::from_millis(heartbeat_interval));
            while let Ok(x) = socket.read_message() {
                println!("Message: {}", x.into_text().unwrap());
            }
            // let json = simd_json::from_slice(&mut x);
        }
    }
}

impl InteractionHandler for WebsocketInteractionHandler {
    fn get_incoming(&self) -> Vec<Interaction> {
        self.receiver.try_iter().collect()
    }
}
