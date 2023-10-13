mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::states::AppState;

use self::systems::{button_interaction, despawn_main_menu, spawn_main_menu};

/// `MainMenuPlugin` handles the main menu's functionality and lifecycle within the game.
///
/// This plugin is responsible for setting up and managing the main menu's systems, ensuring
/// they run at appropriate stages of the game loop. Specifically:
///
/// - `spawn_main_menu` is called when entering the `MainMenu` state, ensuring the main menu UI
///   and elements are appropriately spawned.
/// - `button_interaction` runs continuously during the `Update` stage, handling user interactions
///   with the main menu buttons.
/// - `despawn_main_menu` is called when exiting the `MainMenu` state, ensuring that the main menu
///   UI and elements are cleaned up from the game world.
///
/// # Usage
///
/// To use the `MainMenuPlugin`, you simply add it to your Bevy app during initialization:
///
/// ```rust
/// App::build()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(MainMenuPlugin)
///     .run();
/// ```
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
