mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::{game::states::GameState, states::AppState};

use self::systems::{button_interaction, despawn_pause_menu, spawn_pause_menu};

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
        app.add_systems(
            OnEnter(GameState::Paused),
            spawn_pause_menu.run_if(in_state(AppState::Game)),
        )
        .add_systems(
            Update,
            (button_interaction).run_if(in_state(GameState::Paused)),
        )
        .add_systems(OnExit(GameState::Paused), despawn_pause_menu);
    }
}
