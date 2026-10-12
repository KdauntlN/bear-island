use std::net::SocketAddr;

use wincode::{SchemaRead, SchemaWrite};

pub trait Sendable {
    fn to_framed_bytes(self) -> Vec<u8>;
}

#[derive(Debug, SchemaRead, SchemaWrite)]
pub enum ClientRequest {
    CreateRoom,
    FindRoom {
        code: String,
    },
}

impl Sendable for ClientRequest {
    fn to_framed_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let serialized = wincode::serialize(&self).expect("Failed to serialize message");
        let size = serialized.len().to_be_bytes();

        bytes.extend(&size);
        bytes.extend(&serialized);

        bytes
    }
}

#[derive(Debug, SchemaRead, SchemaWrite)]
pub enum ServerResponse {
    RoomCreated {
        address: SocketAddr,
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