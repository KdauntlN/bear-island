use common::*;
use std::net::SocketAddr;
use bevy::prelude::*;

#[derive(Message, Clone)]
pub enum ClientEvent {
    CreateRoom,
    FindRoom(String),
    JoinRoom(SocketAddr),
}

#[derive(Message)]
pub enum ServerEvent {
    RoomCreated{
        address: SocketAddr
    },
    RoomCreationError{
        error: String
    },
}

impl From<ClientEvent> for ClientRequest {
    fn from(value: ClientEvent) -> Self {
        match value {
            ClientEvent::CreateRoom => Self::CreateRoom,
            ClientEvent::FindRoom(code) => Self::FindRoom { code },
            _ => unreachable!(),
        }
    }
}

impl From<ServerResponse> for ServerEvent {
    fn from(value: ServerResponse) -> Self {
        match value {
            ServerResponse::RoomCreated { address } => Self::RoomCreated { address },
            ServerResponse::RoomCreationError { error } => Self::RoomCreationError { error },
            _ => todo!(),
        }
    }
}