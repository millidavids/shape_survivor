pub mod components;
mod systems;
mod abilities;

use bevy::prelude::*;

use crate::states::AppState;

use self::{systems::{animate_player, camera_follow, despawn_player, move_player, spawn_player}, abilities::AbilitiesPlugin};

use super::states::GameState;

pub const PLAYER_SPEED: f32 = 250.0;

/// `PlayerPlugin`: A Bevy plugin for managing the player's lifecycle, movement, animation, and camera following within the game state.
///
/// This plugin bundles together the systems related to the player entity, ensuring they are registered and executed in the correct order and under the appropriate game states.
///
/// # Systems Managed:
/// * `spawn_player`: Spawns the player entity when entering the `AppState::Game`.
/// * `animate_player`: Animates the player entity during the `Update` phase, but only when the game state is `GameState::Running`.
/// * `move_player`: Moves the player entity based on keyboard input during the `Update` phase, but only when the game state is `GameState::Running`.
/// * `camera_follow`: Makes the camera follow the player's position during the `Update` phase, but only when the game state is `GameState::Running`.
/// * `despawn_player`: Despawns the player entity when exiting the `AppState::Game`.
///
/// # Usage:
/// ```rust
/// fn main() {
///     App::build()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(PlayerPlugin)
///         .run();
/// }
/// ```
/// The above setup ensures that the player entity is managed correctly within the game, from spawning and animation to movement and camera following.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// Configures and adds the player-related systems to the given app.
    ///
    /// * `app`: The Bevy app builder used to register the systems.
    fn build(&self, app: &mut App) {
        app.add_plugins(AbilitiesPlugin)
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (animate_player, move_player, camera_follow).run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
