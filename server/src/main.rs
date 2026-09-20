use common::{ServerMessage, ClientMessage};
use server::room::RoomHandle;

use std::collections::HashMap;
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{io, process};

fn main() {
    let mut rooms: HashMap<String, RoomHandle> = HashMap::new();

    let listener = TcpListener::bind("0.0.0.0:27015").unwrap_or_else(|e| {
        eprintln!("Could not bind TCP listener: {e}");
        process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        if let Err(_) = handle_connection(stream, &mut rooms) {
            continue;
        }
    }
}

fn handle_connection(stream: TcpStream, rooms: &mut HashMap<String, RoomHandle>) -> io::Result<()> {
    let mut buf_reader = BufReader::new(stream);
    let mut size_bytes = [0 as u8; 8];

    buf_reader.read_exact(&mut size_bytes)?;

    let message_size = usize::from_be_bytes(size_bytes);

    let mut message: Vec<u8> = vec![0; message_size];
    buf_reader.read_exact(&mut message)?;

    if let Ok(message) = wincode::deserialize::<ClientMessage>(&message) {
        match message {
            ClientMessage::CreateRoom => {
                let response = match RoomHandle::new() {
                    Ok((mut room, mut code)) => {
                        while rooms.contains_key(&code) {
                            code = room.regenerate_code();
                        }

                        rooms.insert(code.clone(), room);

                        ServerMessage::RoomCreated { code }
                    },
                    Err(e) => {
                        ServerMessage::RoomCreationError { error: e.to_string() }
                    }
                };

                let stream = buf_reader.into_inner();
                let mut buf_writer = BufWriter::new(stream);

                buf_writer.write(&wincode::serialize(&response).expect("Failed to serialise message"))?;
                buf_writer.flush()?;
            }
        }
    } else {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "Data recieved from client was not a valid message"
        ));
    }

    Ok(())
}