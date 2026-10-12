use messages::ClientEvent;
use crate::{game::GameState, ui::messages::ServerEvent};

use bevy::prelude::*;

pub mod messages;
pub mod main_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<ClientEvent>()
            .add_message::<ServerEvent>()
            .add_systems(OnEnter(GameState::MainMenu), main_menu::draw_main_menu)
            .add_systems(Update, (main_menu::quit_button, main_menu::create_room_button).run_if(in_state(GameState::MainMenu)));
    }
}