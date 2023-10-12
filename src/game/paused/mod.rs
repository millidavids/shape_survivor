mod components;
mod systems;
mod styles;

use bevy::prelude::*;

use crate::{game::states::GameState, states::AppState};

use self::systems::{spawn_pause_menu, despawn_pause_menu, button_interaction};

/// A Bevy `Plugin` for managing the paused state of the game.
///
/// The `PausedPlugin` provides the necessary systems to handle the behavior of the game when it's in 
/// the paused state. Specifically, it handles the spawning and interaction with the pause menu, as well
/// as the despawning of the pause menu when the game exits the paused state.
pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    /// Adds systems to the Bevy app to handle the game's paused state.
    ///
    /// # Systems Added:
    /// - On entering the `GameState::Paused` state while the app is in the `AppState::Game` state, the 
    ///   `spawn_pause_menu` system is run to spawn the pause menu entities.
    /// - During the `Update` stage, if the app is in the `GameState::Paused` state, the 
    ///   `button_interaction` system is run to handle button interactions within the pause menu.
    /// - On exiting the `GameState::Paused` state, the `despawn_pause_menu` system is run to remove 
    ///   the pause menu entities from the world.
    ///
    /// # Parameters:
    /// - `app`: A mutable reference to the Bevy `App`, which allows the plugin to add systems.
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu.run_if(in_state(AppState::Game)))
            .add_systems(Update, (button_interaction).run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu);
    }
}