use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::core::{
    abstraction::context::Context,
    interactions::handler::{events::core::HelloPayloadData, gateway_payload::PayloadBase},
};

use super::{
    events::core::HeartBeatPayloadData,
    gateway::{get_gateway, Gateway},
    gateway_payload::PayloadOpcode,
    SocketClient,
};
use async_std::task::block_on;
use crossbeam_channel::{unbounded, Receiver, Sender};

use serde_json::Value;
use simd_json::{self};

use futures_util::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct WebsocketEventHandler {
    event_receiver: Receiver<(String, Value)>,
    command_sender: Sender<Message>,
}

impl WebsocketEventHandler {
    pub async fn create(ctx: Context) -> WebsocketEventHandler {
        let (s, r) = unbounded();
        let (s2, r2) = unbounded();

        let handler = WebsocketEventHandler {
            event_receiver: r,
            command_sender: s2,
        };
        async {
            // TODO so the gateway says that it shouldn't be cached. WHAT DOES THIS MEAN????
            // does it mean not cached between instances, and having it get a new gateway on startup?
            // or does it want use to periodically get a new gateway while the bot is running? plz help
            let gateway = get_gateway(ctx).await.unwrap();
            thread::Builder::new()
                .name("Websocket_Interaction_Handler".to_string())
                .spawn(move || block_on(WebsocketEventHandler::run(s, r2, gateway)))
                .unwrap()
        }
        .await;

        handler
    }

    async fn run(
        event_output: Sender<(String, Value)>,
        incoming_commands: Receiver<Message>,
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
            block_on(WebsocketEventHandler::sender(
                socket_sink,
                incoming_commands,
                socket_to_send,
            ))
        });

        let socket_send1 = socket_send.clone();

        let seq_num_cp = sequence_num.clone();

        // Heartbeat loop
        thread::spawn(move || {
            block_on(WebsocketEventHandler::heartbeat_loop(
                socket_send1,
                hello_payload.data.heartbeat_interval,
                seq_num_cp,
            ))
        });
        // Listen for events, and then send them when they are available
        WebsocketEventHandler::event_receiver(event_output, socket_recv, socket_send, sequence_num)
            .await;
    }

    async fn sender(
        mut socket_send: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        to_send: Receiver<Message>,
        to_send_heartbeat: Receiver<Message>,
    ) {
        // Allowance per second
        let allowance_rate = 120.0 / 60.0;

        let mut allowance: f64 = 120.0;
        loop {
            let start = std::time::Instant::now();

            while let Ok(msg) = to_send_heartbeat.try_recv() {
                if allowance <= 1.0 {
                    break;
                }
                socket_send.send(msg).await.unwrap();
                allowance -= 1.0;
            }

            while let Ok(msg) = to_send.try_recv() {
                if allowance <= 1.0 {
                    break;
                }
                socket_send.send(msg).await.unwrap();
                allowance -= 1.0;
            }

            allowance += start.elapsed().as_secs_f64() * allowance_rate;
            if allowance > 120.0 {
                allowance = 120.0;
            }
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

    async fn event_receiver(
        events: Sender<(String, Value)>,
        mut socket_recv: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        socket_send: Sender<Message>,
        sequence_num: Arc<Mutex<HeartBeatPayloadData>>,
    ) {
        loop {
            let message = socket_recv.next().await.unwrap().unwrap();
            // println!("{}", message);
            let payload: PayloadBase<Value> =
                simd_json::from_slice(&mut *message.into_data()).unwrap();

            match payload.op_code {
                PayloadOpcode::Dispatch => {
                    {
                        *sequence_num.lock().unwrap() = Some(payload.sequence_num.unwrap() as u64);
                    }
                    let event_name = payload.event_name.unwrap();
                    events.send((event_name.to_string(), payload.data)).unwrap();
                    // println!("{}", payload.data)
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

impl SocketClient for WebsocketEventHandler {
    fn send_command(&self, command: String) {
        self.command_sender.send(Message::Text(command)).unwrap();
    }

    fn get_command_channel(&self) -> Receiver<(String, Value)> {
        self.event_receiver.clone()
    }
}
