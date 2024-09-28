mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::{game::states::GameState, states::AppState};

use self::systems::{button_interaction, despawn_level_up_menu, spawn_level_up_menu};

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::LevelUp),
            spawn_level_up_menu.run_if(in_state(AppState::Game)),
        )
        .add_systems(
            Update,
            button_interaction.run_if(in_state(GameState::LevelUp)),
        )
        .add_systems(OnExit(GameState::LevelUp), despawn_level_up_menu);
    }
}
