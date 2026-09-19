use common::{ServerMessage, ClientMessage};

use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::Stdio;
use std::{io, process::{self, Command}};

fn main() {
    let mut streams: Vec<TcpStream> = Vec::new();
    let listener = TcpListener::bind("0.0.0.0:27015").unwrap_or_else(|e| {
        eprintln!("Could not bind TCP listener: {e}");
        process::exit(1);
    });

    println!("Server listening at {}", listener.local_addr().unwrap());

    listener.set_nonblocking(true).unwrap_or_else(|e| {
        eprintln!("Could not set TCP listener to nonblocking {e}");
        process::exit(1);
    });

    loop {
        handle_new_connections(&listener, &mut streams).unwrap_or_else(|e| {
            eprintln!("Error accepting new connection: {e}");
            process::exit(1);
        });

        let messages = check_for_messages(&mut streams).unwrap_or_else(|e| {
            eprintln!("Error from client: {e}");
            process::exit(1);
        });

        for (message, address) in messages {
            match message {
                ClientMessage::CreateRoom => {
                    if let Ok(room_addr) = create_room() {
                        send_message(&mut streams, address, ServerMessage::RoomCreated { address: room_addr }).unwrap_or_else(|e| {
                            eprintln!("Failed to send message to client: {e}");
                            process::exit(1);
                        })
                    }
                }
            }
        }
    }
}

fn create_room() -> io::Result<String> {
    println!("Attempting to create room");
    let mut room_host = Command::new("./room_host")
        .stdout(Stdio::piped())
        .spawn()?;

    println!("Room process spawned");

    let stdout = room_host.stdout.take().ok_or(io::Error::new(ErrorKind::BrokenPipe, "Could not open child stdout"))?;
    let mut reader = BufReader::new(stdout);
    let mut port = String::new();

    reader.read_line(&mut port)?;

    let mut address = String::from("152.69.167.180:");
    address.push_str(port.trim());

    Ok(address)
}

fn handle_new_connections(listener: &TcpListener, streams: &mut Vec<TcpStream>) -> io::Result<()> {
    for result in listener.incoming() {
        match result {
            Ok(stream) => {
                stream.set_nonblocking(true)?;
                streams.push(stream);
                println!("New connection");
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => break,
            Err(e) => Err(e)?,
        }
    }

    Ok(())
}

fn check_for_messages(streams: &mut Vec<TcpStream>) -> io::Result<Vec<(ClientMessage, SocketAddr)>> {
    let mut messages: Vec<(ClientMessage, SocketAddr)> = Vec::new();
    let mut closed_streams: Vec<usize> = Vec::new();
    let mut buffer = [0 as u8; 128];

    for (i, stream) in streams.iter_mut().enumerate() {
        match stream.peek(&mut buffer) {
            Ok(0) => closed_streams.push(i),
            Ok(_) => {
                stream.read(&mut buffer)?;
                let message = wincode::deserialize::<ClientMessage>(&buffer).expect("Invalid message");
                println!("Recieved message: {message:?}");
                messages.push((message, stream.local_addr()?));
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
            Err(e) => Err(e)?
        }
    }

    for i in &closed_streams {
        streams.remove(closed_streams[*i]);
    }

    Ok(messages)
}

fn send_message(streams: &mut Vec<TcpStream>, addr: SocketAddr, message: ServerMessage) -> io::Result<()> {
    for stream in streams {
        if stream.local_addr()? == addr {
            stream.write(&wincode::serialize(&message).expect("Failed to serialize message"))?;
            stream.flush()?;
        }
    }

    Ok(())
}