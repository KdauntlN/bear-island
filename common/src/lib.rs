use std::net::SocketAddr;

use wincode::{SchemaRead, SchemaWrite};

#[derive(Debug, SchemaRead, SchemaWrite)]
pub enum ClientMessage {
    CreateRoom,
    JoinRoom {
        code: String,
    }
}

#[derive(Debug, SchemaRead, SchemaWrite)]
pub enum ServerMessage {
    RoomCreated {
        code: String,
    },
    RoomCreationError {
        error: String,
    },
    RoomFound {
        address: SocketAddr,
    },
    RoomJoinError {
        reason: JoinError,
    },
}

#[derive(Debug, SchemaRead, SchemaWrite)]
pub enum JoinError {
    RoomFull,
    RoomDoesntExist,
}