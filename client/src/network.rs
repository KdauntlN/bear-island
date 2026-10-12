use crate::ui::messages::*;
use common::*;

use std::io::{Write, Read, BufReader};
use std::net::TcpStream;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;

use bevy::prelude::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (send_messages, receive_messages));
    }
}

#[derive(Resource)]
struct NetworkThreadHandler {
    _tcp_thread: JoinHandle<()>,
    _udp_thread: JoinHandle<()>,

    tcp_tx: Sender<ClientEvent>,
    tcp_rx: Mutex<Receiver<ServerEvent>>,

    udp_tx: Sender<ClientEvent>,
    udp_rx: Mutex<Receiver<ServerEvent>>,
}

impl NetworkThreadHandler {
    fn new() -> Self {
        let (tcp_tx_a, tcp_rx_a) = mpsc::channel::<ClientEvent>();
        let (tcp_tx_b, tcp_rx_b) = mpsc::channel::<ServerEvent>();

        let (udp_tx_a, udp_rx_a) = mpsc::channel::<ClientEvent>();
        let (udp_tx_b, udp_rx_b) = mpsc::channel::<ServerEvent>();

        let _tcp_thread = thread::spawn(move || {
            let tcp_handler = TcpHandler::new(tcp_tx_b, tcp_rx_a);
            tcp_handler.run();
        });

        let _udp_thread = thread::spawn(|| {

        });

        Self {
            _tcp_thread,
            _udp_thread,

            tcp_tx: tcp_tx_a,
            tcp_rx: Mutex::new(tcp_rx_b),

            udp_tx: udp_tx_a,
            udp_rx: Mutex::new(udp_rx_b),
        }
    }
}

struct TcpHandler {
    stream: TcpStream,
    tx: Sender<ServerEvent>,
    rx: Receiver<ClientEvent>,
}

impl TcpHandler {
    fn new(tx: Sender<ServerEvent>, rx: Receiver<ClientEvent>) -> Self {
        Self {
            stream: TcpStream::connect("152.69.167.180:27015").expect("Failed to connect to server"),
            tx,
            rx
        }
    }

    fn run(mut self) {
        loop {
            if let Some(message) = self.rx.try_recv().ok() {
                if let ClientEvent::JoinRoom(address) = message {
                    // hopefully connect to room
                    self.stream = TcpStream::connect(address).unwrap_or_else(|e| {
                        // try to connect back to server as fallback
                        eprintln!("Failed to connect to room: {e}");
                        TcpStream::connect("152.69.167.180:27015").expect("Failed to connect to server")
                    });

                    if let Some(response) = self.check_connection(&self.stream) {
                        let message = ServerEvent::from(response);
                        self.tx.send(message).expect("Failed to send message");
                    }

                    continue;
                }

                let request = ClientRequest::from(message);
                let bytes = request.to_framed_bytes();
                self.stream.write_all(&bytes).expect("Failed to write bytes");
                self.stream.flush().expect("Failed to flush stream");

                if let Some(response) = self.check_connection(&self.stream) {
                    let message = ServerEvent::from(response);
                    self.tx.send(message).expect("Failed to send message");
                }
            }
        }
    }

    fn check_connection(&self, stream: &TcpStream) -> Option<ServerResponse> {
        let mut size_bytes = [0u8; 8];
        let mut buf_reader = BufReader::new(stream);

        buf_reader.read_exact(&mut size_bytes).ok()?;

        let size = usize::from_be_bytes(size_bytes);
        let mut message = vec![0u8; size];

        buf_reader.read_exact(&mut message).ok()?;

        wincode::deserialize(&message).ok()?
    }
}

pub struct UdpHandler {

}

fn setup(
    mut commands: Commands,
) {
    let network_handler = NetworkThreadHandler::new();
    commands.insert_resource(network_handler);
}

fn send_messages(mut reader: MessageReader<ClientEvent>, handler: Res<NetworkThreadHandler>) {
    for message in reader.read() {
        use ClientEvent as C;

        match message {
            C::CreateRoom => handler.tcp_tx.send(message.clone()).expect("Failed to send message"),
            C::FindRoom(_) => handler.tcp_tx.send(message.clone()).expect("Failed to send message"),
            C::JoinRoom(_) => handler.tcp_tx.send(message.clone()).expect("Failed to send message"),
            _ => panic!()
        }
    }
}

fn receive_messages(mut writer: MessageWriter<ServerEvent>, handler: Res<NetworkThreadHandler>) {
    if let Ok(lock) = handler.tcp_rx.lock() {
        let Ok(message) = lock.try_recv() else { return; };
        writer.write(message);
    }

    if let Ok(lock) = handler.udp_rx.lock() {
        let Ok(message) = lock.try_recv() else { return; };
        writer.write(message);
    }
}