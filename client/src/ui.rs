use messages::UiEvent;
use crate::game::GameState;

use bevy::prelude::*;

pub mod messages;
pub mod main_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<UiEvent>()
            .add_systems(OnEnter(GameState::MainMenu), main_menu::draw_main_menu);
    }
}