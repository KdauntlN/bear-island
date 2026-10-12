use room_host::Player;
use common::*;
use std::{io::{self, Write}, net::TcpListener, process};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let listener = match bind_listener() {
        Some(listener) => listener,
        None => {
            print!("\n");
            process::exit(1);
        },
    };

    // Tell server the port picked
    print!("{}\n", listener.local_addr()?.port());

    let mut code = String::new();

    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read code");

    let mut players: HashMap<usize, Player> = HashMap::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let next_id = players
            .keys()
            .max()
            .and_then(|id| Some(*id))
            .unwrap_or(1);

        let player = Player { stream: stream.try_clone().expect("Failed to clone stream") };

        players.insert(next_id, player);

        stream.write_all(&ServerResponse::RoomJoined { info: RoomInfo { code: code.clone() } }.to_framed_bytes()).expect("Failed to send message");
        stream.flush().unwrap();
    }

    Ok(())
}

fn bind_listener() -> Option<TcpListener> {
    for port in 27016..27115 {
        if let Ok(listener) = TcpListener::bind(("0.0.0.0", port)) {
            return Some(listener);
        } else {
            continue;
        }
    }

    None
}