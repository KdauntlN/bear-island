use client::{
    ui::UiPlugin,
    network::NetworkPlugin,
    game::GamePlugin,
    lobby::LobbyPlugin,
};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                exit_condition: bevy::window::ExitCondition::DontExit,
                ..default()
            }),
            UiPlugin,
            NetworkPlugin,
            GamePlugin,
            LobbyPlugin,
        ))
        .run();
}