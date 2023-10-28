pub mod abilities;
pub mod components;
pub mod events;
mod systems;

use bevy::prelude::*;

use crate::states::AppState;

use self::{
    abilities::AbilitiesPlugin,
    events::PlayerLevelUpEvent,
    systems::{add_xp, camera_follow, despawn_player, move_player, spawn_player},
};

use super::states::GameState;

pub const PLAYER_SPEED: f32 = 250.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// Configures and adds the player-related systems to the given app.
    ///
    /// * `app`: The Bevy app builder used to register the systems.
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerLevelUpEvent>()
            .add_plugins(AbilitiesPlugin)
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                ((move_player, camera_follow).chain(), add_xp)
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
