mod systems;
mod components;

use bevy::prelude::*;

use crate::states::AppState;

use self::systems::{spawn_player, animate_player, despawn_player};

use super::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(Update, animate_player.run_if(in_state(GameState::Running)))
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}