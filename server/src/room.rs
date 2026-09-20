use std::{
    io::{self, BufRead, BufReader}, net::SocketAddr, process::{Command, Stdio},
};

use rand::RngExt;

const ALPHABET: &[u8] = b"ABCDEFGHIJMNPQRSTUVWXYZ23456789";

pub struct RoomHandle {
    room_code: String,
    address: SocketAddr,
    player_count: u16,
    alive: bool,
}

impl RoomHandle {
    pub fn new() -> io::Result<(Self, String)> {
        let mut room_host = Command::new("./room_host")
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = room_host
            .stdout
            .take()
            .expect("Stdout was not piped");

        let mut reader = BufReader::new(stdout);
        let mut room_port = String::new();
        reader.read_line(&mut room_port)?;

        let address = format!("152.69.167.180:{}", room_port.trim()).parse::<SocketAddr>().expect("Failed to parse address");
        let room_code = generate_room_code();

        return Ok((
            Self {
                room_code: room_code.clone(),
                address,
                player_count: 0,
                alive: false,
            },
            room_code
        ))
    }

    pub fn regenerate_code(&mut self) -> String {
        let new_code = generate_room_code();
        self.room_code = new_code.clone();
        new_code
    }
}

fn generate_room_code() -> String {
    let mut rnd = rand::rng();

    (0..6).map(|_| {
        let i = rnd.random_range(0..ALPHABET.len());
        ALPHABET[i] as char
    })
    .collect()
}