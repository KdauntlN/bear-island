use crate::ui::messages::*;
use crate::GameEvent;
use common::*;

use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;

use bevy::prelude::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, send_ui_messages);
    }
}

#[derive(Resource)]
struct TcpThread {
    thread: JoinHandle<()>,
    tx: Sender<Box<dyn GameEvent + Send>>,
    rx: Mutex<Receiver<ServerMessage>>,
}

impl TcpThread {
    fn new() -> Self {
        let (tx_a, rx_a) = mpsc::channel::<Box<dyn GameEvent + Send>>();
        let (tx_b, rx_b) = mpsc::channel::<ServerMessage>();

        let thread = thread::spawn(move || {
            let tcp_handler = TcpHandler::new(tx_b, rx_a);
            tcp_handler.run();
        });

        Self {
            thread,
            tx: tx_a,
            rx: Mutex::new(rx_b),
        }
    }
}

struct TcpHandler {
    tx: Sender<ServerMessage>,
    rx: Receiver<Box<dyn GameEvent + Send>>,
}

impl TcpHandler {
    fn new(tx: Sender<ServerMessage>, rx: Receiver<Box<dyn GameEvent + Send>>) -> Self {
        Self {
            tx,
            rx
        }
    }

    fn run(self) {
        loop {
            todo!();
        }
    }
}

fn setup(
    mut commands: Commands,
) {
    let tcp_thread = TcpThread::new();
    commands.insert_resource(tcp_thread);
}

fn send_ui_messages(mut reader: MessageReader<UiEvent>, handler: Res<TcpThread>) {
    for message in reader.read() {
        handler.tx.send(Box::new(message.clone())).expect("Failed to send message");
    }
}