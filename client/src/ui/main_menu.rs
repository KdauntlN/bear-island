use bevy::prelude::*;
use crate::game::{ButtonTextures, GameState};

#[derive(Component)]
pub struct CreateRoomButton;

#[derive(Component)]
pub struct JoinRoomButton;

#[derive(Component)]
pub struct QuitButton;

pub fn draw_main_menu(
    mut commands: Commands,
    button_textures: Res<ButtonTextures>,
) {
    commands.spawn((
        DespawnOnExit(GameState::MainMenu),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        }
    )).with_children(|parent| {
        parent.spawn((
            CreateRoomButton,
            Button,
            ImageNode::new(button_textures.create_room.clone()),
            Node {
                width: Val::Px(344.0),
                height: Val::Px(92.0),
                ..default()
            }
        ));

        parent.spawn((
            JoinRoomButton,
            Button,
            ImageNode::new(button_textures.join_room.clone()),
            Node {
                width: Val::Px(344.0),
                height: Val::Px(92.0),
                ..default()
            }
        ));

        parent.spawn((
            QuitButton,
            Button,
            ImageNode::new(button_textures.quit.clone()),
            Node {
                width: Val::Px(344.0),
                height: Val::Px(92.0),
                ..default()
            }
        ));
    });
}