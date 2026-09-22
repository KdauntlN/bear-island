use crate::GameEvent;

use bevy::prelude::*;

#[derive(Message, Clone)]
pub enum UiEvent {
    CreateRoom,
}

impl GameEvent for UiEvent {

}