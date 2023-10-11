pub mod components;
mod systems;

use bevy::prelude::*;

use crate::states::AppState;

use self::systems::{animate_player, camera_follow, despawn_player, move_player, spawn_player};

use super::states::GameState;

pub const PLAYER_SPEED: f32 = 250.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (animate_player, move_player, camera_follow).run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
