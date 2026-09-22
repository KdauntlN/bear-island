use crate::ui::messages::UiEvent;

use bevy::prelude::*;

pub mod messages;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<UiEvent>();
    }
}