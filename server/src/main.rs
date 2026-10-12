use common::{ServerResponse, ClientRequest, JoinError};
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
        println!("New connection");

        if let Err(e) = handle_connection(stream, &mut rooms) {
            eprintln!("There was an error with the connection: {e};");
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

    if let Ok(message) = wincode::deserialize::<ClientRequest>(&message) {
        let response = match message {
            ClientRequest::CreateRoom => {
                match RoomHandle::new() {
                    Ok((mut room, mut code)) => {
                        while rooms.contains_key(&code) {
                            code = room.regenerate_code();
                        }

                        let address = room.addr();

                        rooms.insert(code.clone(), room);

                        ServerResponse::RoomCreated { address }
                    },
                    Err(e) => {
                        ServerResponse::RoomCreationError { error: e.to_string() }
                    }
                }
            },
            ClientRequest::JoinRoom { code } => {
                match rooms.get(&code) {
                    Some(room) if room.players < 6 => {
                        ServerResponse::RoomFound { address: room.address }
                    },
                    Some(_) => {
                        ServerResponse::RoomJoinError { reason: JoinError::RoomFull }
                    },
                    None => {
                        ServerResponse::RoomJoinError { reason: JoinError::RoomFull }
                    }
                }
            }
        };

        let stream = buf_reader.into_inner();
        let mut buf_writer = BufWriter::new(stream);

        let message = wincode::serialize(&response).expect("Failed to serialise message");
        let size = message.len().to_be_bytes();

        buf_writer.write(&size)?;
        buf_writer.write(&message)?;
        buf_writer.flush()?;
    } else {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "Data recieved from client was not a valid message"
        ));
    }

    Ok(())
}