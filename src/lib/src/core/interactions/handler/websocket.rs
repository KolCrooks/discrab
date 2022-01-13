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

use futures_util::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use serde_json::Value;
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
        // Url of the websocket
        let url = url::Url::parse(&format!("{}/?v=9&encoding=json", gateway.url)).unwrap();
        // println!("Connecting to {}", url);

        // Connect to the websocket
        let (mut socket, _) = connect_async(url).await.expect("Can't connect");
        // println!("{}", response.status());

        // Receive the hello message from the websocket and then parse it
        let mut hello_msg = socket.next().await.unwrap().unwrap().into_data();

        let hello_payload: PayloadBase<HelloPayloadData> =
            serde_json::from_slice(&mut *hello_msg).unwrap();

        // Split the socket so that different threads can handle different parts of the websocket
        let (socket_sink, socket_recv) = socket.split();

        // Used to send messages to the websocket
        let (heartbeat_send, heartbeat_receiver) = unbounded();

        // The Sequence number. See https://discord.com/developers/docs/topics/gateway#heartbeat
        let sequence_num = Arc::new(Mutex::new(None));

        // This will send requests to the websocket that are sent through the incoming_commands channel and the heartbeat_receiver channel
        thread::spawn(move || {
            block_on(WebsocketEventHandler::sender(
                socket_sink,
                incoming_commands,
                heartbeat_receiver,
            ))
        });

        let heatbeat_send1 = heartbeat_send.clone();
        let seq_num_cp = sequence_num.clone();

        // Heartbeat loop
        thread::spawn(move || {
            block_on(WebsocketEventHandler::heartbeat_loop(
                heatbeat_send1,
                hello_payload.data.heartbeat_interval,
                seq_num_cp,
            ))
        });

        // Listen for events, and then send them when they are available
        WebsocketEventHandler::event_receiver(
            event_output,
            socket_recv,
            heartbeat_send,
            sequence_num,
        )
        .await;
    }

    /// This will send requests to the websocket that are sent through the incoming_commands channel and the heartbeat_receiver channel
    async fn sender(
        mut socket_send: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        to_send: Receiver<Message>,
        to_send_heartbeat: Receiver<Message>,
    ) {
        let max_allowance = 120.0;
        // Allowance per second
        let allowance_rate = max_allowance / 60.0;

        // The current allowance
        let mut allowance: f64 = max_allowance;

        // This combines the two channels into one for signaling when there are messages to send.
        // This is to block the thread when it doesn't need to do anything
        let mut sel = crossbeam_channel::Select::new();

        sel.recv(&to_send);
        sel.recv(&to_send_heartbeat);

        // This loop will handle the messages
        loop {
            let start = std::time::Instant::now();

            // Block the thread until there is a message to send
            sel.ready();

            // Send the heartbeat if there is allowance to do so, and if there is a heartbeat message to send
            while let Ok(msg) = to_send_heartbeat.try_recv() {
                if allowance <= 1.0 {
                    break;
                }
                socket_send.send(msg).await.unwrap();
                allowance -= 1.0;
            }

            // Send the message if there is allowance to do so, and if there is a message to send
            while let Ok(msg) = to_send.try_recv() {
                if allowance <= 1.0 {
                    break;
                }
                socket_send.send(msg).await.unwrap();
                allowance -= 1.0;
            }

            // Calculate the new allowance
            allowance += start.elapsed().as_secs_f64() * allowance_rate;
            if allowance > max_allowance {
                allowance = max_allowance;
            }
        }
    }

    /// Sends a heartbeat to the websocket every `heartbeat_interval` seconds
    async fn heartbeat_loop(
        socket_send: Sender<Message>,
        heartbeat_interval: u64,
        sequence_num: Arc<Mutex<HeartBeatPayloadData>>,
    ) {
        loop {
            let seq = *sequence_num.lock().unwrap();
            let heartbeat = Message::Text(serde_json::to_string(&PayloadBase::new(seq)).unwrap());
            socket_send.send(heartbeat).unwrap();
            thread::sleep(std::time::Duration::from_millis(heartbeat_interval));
        }
    }

    /// Will receive events from the websocket and send them to the event_output channel
    async fn event_receiver(
        events: Sender<(String, Value)>,
        mut socket_recv: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        socket_send: Sender<Message>,
        sequence_num: Arc<Mutex<HeartBeatPayloadData>>,
    ) {
        // Listen for the socket to receive a message
        while let Ok(message) = socket_recv.next().await.unwrap() {
            // Parse the payload
            let payload: PayloadBase<Value> =
                serde_json::from_slice(&mut *message.into_data()).unwrap();

            // Handle the payload depending on the opcode
            match payload.op_code {
                PayloadOpcode::Dispatch => {
                    // Update the sequence number
                    {
                        *sequence_num.lock().unwrap() = Some(payload.sequence_num.unwrap() as u64);
                    }
                    let event_name = payload.event_name.unwrap();
                    events.send((event_name.to_string(), payload.data)).unwrap();
                }
                PayloadOpcode::Heartbeat => {
                    // Send a heartbeat if it is requested
                    let seq = *sequence_num.lock().unwrap();
                    let heartbeat =
                        Message::Text(serde_json::to_string(&PayloadBase::new(seq)).unwrap());
                    socket_send.send(heartbeat).unwrap();
                }
                PayloadOpcode::Reconnect => {}
                PayloadOpcode::InvalidSession => {}
                PayloadOpcode::Hello => {
                    // This shouldn't happen so it is weird that we are in this branch
                }
                PayloadOpcode::HeartbeatAck => {
                    // Acknowledged heartbeat
                    // println!("Heartbeat acknowledged");
                }
                _ => {}
            }
            // interactions.send(interaction).await.unwrap();
        }
    }
}

impl SocketClient for WebsocketEventHandler {
    // Sends a command through the websocket client
    fn send_command(&self, command: String) {
        self.command_sender.send(Message::Text(command)).unwrap();
    }

    // Get the command channel associated with the socket client
    fn get_command_channel(&self) -> Receiver<(String, Value)> {
        self.event_receiver.clone()
    }
}
