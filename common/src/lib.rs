use wincode::{SchemaRead, SchemaWrite};

#[derive(Debug, SchemaRead, SchemaWrite)]
pub enum ClientMessage {
    CreateRoom,
}

#[derive(Debug, SchemaRead, SchemaWrite)]
pub enum ServerMessage {
    RoomCreated {
        address: String,
    },
}