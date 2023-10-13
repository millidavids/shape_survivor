mod game;
mod main_menu;
mod states;
mod systems;

use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use states::AppState;
use systems::{spawn_camera, toggle_app_state};

/// Entry point for the Bevy-based game application.
///
/// Initializes and runs the Bevy game application with a series of configurations, plugins, and systems.
///
/// # Features
///
/// 1. **ClearColor**: Sets the default clear color of the window to a light beige hue.
/// 2. **DefaultPlugins**: Adds the default set of Bevy plugins to the application.
/// 3. **GameState**: Registers the `AppState` enum to manage different application states like `MainMenu` and `Game`.
/// 4. **Plugins**: Adds custom plugins - `MainMenuPlugin` for handling main menu related logic, and `GamePlugin` for the main gameplay.
/// 5. **Systems**:
///     - At startup, the `spawn_camera` system is added, which centers a 2D camera in the primary window.
///     - During each update, the `toggle_app_state` system checks for specific key presses to toggle between game states.
///
/// # Usage
///
/// Simply run the binary of the game crate to launch the application. The `main` function will handle the initialization and game loop.
///
/// ```
/// cargo run --release
/// ```
///
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Hsla {
            hue: 56.0,
            saturation: 0.09,
            lightness: 0.96,
            alpha: 1.0,
        }))
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, toggle_app_state)
        .run();
}
