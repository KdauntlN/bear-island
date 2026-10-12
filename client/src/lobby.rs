use bevy::prelude::*;

use crate::game::GameState;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_lobby_ui.run_if(in_state(GameState::Lobby)));
    }
}

fn draw_lobby_ui(mut commands: Commands) {
    
}