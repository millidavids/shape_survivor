mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::states::AppState;

use self::systems::{button_interaction, despawn_main_menu, spawn_main_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    /// Sets up the systems required for the main menu's functionality.
    ///
    /// This method adds systems to the Bevy app, binding them to specific stages of the game loop:
    ///
    /// - On entering the `MainMenu` state, it triggers the `spawn_main_menu` system to spawn
    ///   the UI elements of the main menu.
    /// - During the `Update` stage, the `button_interaction` system continuously checks for
    ///   and handles user interactions with the main menu buttons.
    /// - On exiting the `MainMenu` state, it triggers the `despawn_main_menu` system to
    ///   clean up the main menu's UI elements from the game world.
    ///
    /// # Parameters
    ///
    /// - `app`: A mutable reference to the main Bevy app. This allows for integrating and
    ///   configuring systems and resources within the app.
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, button_interaction)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
