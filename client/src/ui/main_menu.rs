use bevy::prelude::*;
use crate::game::{UiTextures, GameState, LoadingState};
use super::messages::ClientEvent;

#[derive(Component)]
pub struct CreateRoomButton;

#[derive(Component)]
pub struct JoinRoomButton;

#[derive(Component)]
pub struct QuitButton;

pub fn draw_main_menu(
    mut commands: Commands,
    ui_textures: Res<UiTextures>,
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
            ImageNode::new(ui_textures.logo_text.clone()),
            Node {
                width: Val::Px(940.8),
                height: Val::Px(529.2),
                ..default()
            },
        ));

        parent.spawn((
            CreateRoomButton,
            Button,
            ImageNode::new(ui_textures.create_room.clone()),
            Node {
                width: Val::Px(344.0),
                height: Val::Px(92.0),
                ..default()
            }
        ));

        parent.spawn((
            JoinRoomButton,
            Button,
            ImageNode::new(ui_textures.join_room.clone()),
            Node {
                width: Val::Px(344.0),
                height: Val::Px(92.0),
                ..default()
            }
        ));

        parent.spawn((
            QuitButton,
            Button,
            ImageNode::new(ui_textures.quit.clone()),
            Node {
                width: Val::Px(344.0),
                height: Val::Px(92.0),
                ..default()
            }
        ));
    });
}

pub fn quit_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut message_writer: MessageWriter<AppExit>
) {
    for interaction in button_query {
        if let Interaction::Pressed = *interaction {
            message_writer.write(AppExit::Success);
        }
    }
}

pub fn create_room_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<CreateRoomButton>)>,
    mut message_writer: MessageWriter<ClientEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_substate: Option<ResMut<NextState<LoadingState>>>,
) {
    for interaction in button_query {
        if let Interaction::Pressed = *interaction {
            message_writer.write(ClientEvent::CreateRoom);
            next_state.set(GameState::Loading);
            if let Some(ref mut next_substate) = next_substate {
                next_substate.set(LoadingState::CreatingRoom);
            }
        }
    }
}

/*
pub fn join_room_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut message_writer: MessageWriter<UiMessage>
) {
    for interaction in button_query {
        if let Interaction::Pressed = *interaction {
            message_writer.write();
        }
    }
}
*/