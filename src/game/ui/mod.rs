mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::states::AppState;

use self::systems::{animate_xp_bar, animate_health_bar, despawn_ui, spawn_ui, update_xp_text};

use super::states::GameState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_ui)
            .add_systems(
                Update,
                (animate_health_bar, animate_xp_bar, update_xp_text).run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_ui);
    }
}
