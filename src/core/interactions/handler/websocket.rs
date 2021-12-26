use std::{
    sync::{atomic::AtomicI64, Arc, Mutex},
    thread,
};

use crate::core::{
    http::rate_limit_client::RLClient,
    interactions::{
        handler::{events::core::HelloPayloadData, gateway_payload::PayloadBase},
        typing::Interaction,
    },
};

use super::{
    events::core::HeartBeatPayloadData,
    gateway::{get_gateway, Gateway},
    gateway_payload::PayloadOpcode,
    InteractionHandler,
};
use async_std::task::block_on;
use crossbeam_channel::{unbounded, Receiver, Sender};

use serde_json::Value;
use simd_json::{self};

use futures_util::{
    stream::{SplitSink, SplitStream, StreamExt},
    Sink, SinkExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct WebsocketInteractionHandler {
    interaction_receiver: Receiver<Interaction>,
    command_sender: Sender<String>,
    thread_handle: thread::JoinHandle<()>,
}

impl WebsocketInteractionHandler {
    pub async fn create(http_client: &RLClient) -> WebsocketInteractionHandler {
        let (s, r) = unbounded();
        let (s2, r2) = unbounded();

        let handler = WebsocketInteractionHandler {
            interaction_receiver: r,
            command_sender: s2,
            thread_handle: async {
                // TODO so the gateway says that it shouldn't be cached. WHAT DOES THIS MEAN????
                // does it mean not cached between instances, and having it get a new gateway on startup?
                // or does it want use to periodically get a new gateway while the bot is running? plz help
                let gateway = get_gateway(http_client).await.unwrap();
                thread::Builder::new()
                    .name("Websocket_Interaction_Handler".to_string())
                    .spawn(move || block_on(WebsocketInteractionHandler::run(s, r2, gateway)))
                    .unwrap()
            }
            .await,
        };
        handler
    }

    async fn run(
        interaction_output: Sender<Interaction>,
        incomming_commands: Receiver<String>,
        gateway: Gateway,
    ) {
        let url = url::Url::parse(&format!("{}/?v=9&encoding=json", gateway.url)).unwrap();
        // println!("Connecting to {}", url);

        let (mut socket, _) = connect_async(url).await.expect("Can't connect");
        // println!("{}", response.status());

        let mut hello_msg = socket.next().await.unwrap().unwrap().into_data();

        let hello_payload: PayloadBase<HelloPayloadData> =
            simd_json::from_slice(&mut *hello_msg).unwrap();

        let (socket_sink, socket_recv) = socket.split();
        let (socket_send, socket_to_send) = unbounded();

        let sequence_num = Arc::new(Mutex::new(None));

        // This is required so that everything is forwarded through here
        thread::spawn(move || {
            block_on(WebsocketInteractionHandler::sender(
                socket_sink,
                socket_to_send,
            ))
        });

        let socket_send1 = socket_send.clone();
        let socket_send2 = socket_send.clone();

        let seq_num_cp = sequence_num.clone();

        // We want 3 things
        // 1. Heartbeat loop
        thread::spawn(move || {
            block_on(WebsocketInteractionHandler::heartbeat_loop(
                socket_send1,
                hello_payload.data.heartbeat_interval,
                seq_num_cp,
            ))
        });
        // 2. Listen for commands, and then send them when they are available
        thread::spawn(move || {
            block_on(WebsocketInteractionHandler::command_handler(
                incomming_commands,
                socket_send2,
            ))
        });
        // 3. Listen for interactions, and then send them when they are available
        WebsocketInteractionHandler::interaction_receiver(
            interaction_output,
            socket_recv,
            socket_send,
            sequence_num,
        )
        .await;
    }

    async fn sender(
        mut socket_send: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        to_send: Receiver<Message>,
    ) {
        while let Ok(msg) = to_send.recv() {
            socket_send.send(msg).await.unwrap();
        }
    }

    async fn heartbeat_loop(
        socket_send: Sender<Message>,
        interval: u64,
        sequence_num: Arc<Mutex<HeartBeatPayloadData>>,
    ) {
        loop {
            let seq = *sequence_num.lock().unwrap();
            let heartbeat = Message::Text(simd_json::to_string(&PayloadBase::new(seq)).unwrap());
            socket_send.send(heartbeat).unwrap();
            thread::sleep(std::time::Duration::from_millis(interval));
        }
    }

    async fn command_handler(commands: Receiver<String>, socket_send: Sender<Message>) {
        while let Ok(command) = commands.recv() {
            let message = Message::Text(command);
            socket_send.send(message).unwrap();
        }
    }

    async fn interaction_receiver(
        interactions: Sender<Interaction>,
        mut socket_recv: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        socket_send: Sender<Message>,
        sequence_num: Arc<Mutex<HeartBeatPayloadData>>,
    ) {
        loop {
            let message = socket_recv.next().await.unwrap().unwrap();
            println!("{}", message);
            let payload: PayloadBase<Value> =
                simd_json::from_slice(&mut *message.into_data()).unwrap();

            match payload.op_code {
                PayloadOpcode::Dispatch => {
                    {
                        *sequence_num.lock().unwrap() = Some(payload.sequence_num.unwrap() as u64);
                    }
                    println!("{}", payload.data)
                }
                PayloadOpcode::Heartbeat => {
                    let seq = *sequence_num.lock().unwrap();
                    let heartbeat =
                        Message::Text(simd_json::to_string(&PayloadBase::new(seq)).unwrap());
                    socket_send.send(heartbeat).unwrap();
                }
                PayloadOpcode::Reconnect => {}
                PayloadOpcode::InvalidSession => {}
                PayloadOpcode::Hello => {
                    // This shouldn't happen so it is weird that we are in this branch
                }
                PayloadOpcode::HeartbeatAck => {
                    // Acknowledged heartbeat
                    println!("Heartbeat acknowledged");
                }
                _ => {}
            }
            // interactions.send(interaction).await.unwrap();
        }
    }
}

impl InteractionHandler for WebsocketInteractionHandler {
    fn get_incoming(&self) -> Vec<Interaction> {
        self.interaction_receiver.try_iter().collect()
    }

    fn send_command(&self, command: String) {
        self.command_sender.send(command).unwrap();
    }
}
