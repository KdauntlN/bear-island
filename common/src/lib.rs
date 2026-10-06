use std::net::SocketAddr;

use wincode::{SchemaRead, SchemaWrite};

#[derive(Debug, SchemaRead, SchemaWrite)]
pub enum ClientRequest {
    CreateRoom,
    JoinRoom {
        code: String,
    }
}

#[derive(Debug, SchemaRead, SchemaWrite)]
pub enum ServerResponse {
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