mod components;
mod enemies;
mod grid;
mod paused;
mod player;
mod states;
mod systems;

use bevy::prelude::*;

use crate::{states::AppState, systems::push_main_menu};

use self::{
    grid::GridPlugin,
    paused::PausedPlugin,
    player::PlayerPlugin,
    states::GameState,
    systems::{deactivate_game, pause_game, toggle_game_state}, enemies::EnemiesPlugin,
};

/// `GamePlugin` manages the core game functionality and its lifecycle.
///
/// This plugin is responsible for setting up and managing the game's core systems and states, 
/// ensuring they run at appropriate stages of the game loop. Specifically, it:
///
/// - Sets up the main game state via `GameState`.
/// - Adds various game-related plugins such as `PausedPlugin`, `PlayerPlugin`, `GridPlugin`, and `EnemiesPlugin`.
/// - Manages game state transitions and associated system executions. For instance, it ensures 
///   the game is paused upon entering the `Game` state, toggles the game state based on certain 
///   conditions, and handles the transition back to the main menu from the `Inactive` state.
///
/// # Usage
///
/// To use the `GamePlugin`, you simply add it to your Bevy app during initialization:
///
/// ```rust
/// App::build()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(GamePlugin)
///     .run();
/// ```
pub struct GamePlugin;

impl Plugin for GamePlugin {
    /// Sets up the systems and states essential for the game's functionality.
    ///
    /// This method integrates systems and states into the Bevy app, binding them to specific 
    /// stages of the game loop and conditions:
    ///
    /// - Initializes the `GameState` as the main state for gameplay.
    /// - Adds game-centric plugins for managing various gameplay aspects.
    /// - Triggers the `pause_game` system upon entering the `Game` state.
    /// - Continuously runs the `toggle_game_state` system during the `Update` stage, but only 
    ///   if the current app state is `Game`.
    /// - On entering the `Inactive` game state, it triggers the `push_main_menu` system to 
    ///   transition to the main menu.
    /// - On exiting the `Game` app state, it triggers the `deactivate_game` system, but avoids 
    ///   running this system if the game state is already `Inactive`.
    ///
    /// # Parameters
    ///
    /// - `app`: A mutable reference to the main Bevy app. This allows for integrating and
    ///   configuring systems, states, and plugins within the app.
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((PausedPlugin, PlayerPlugin, GridPlugin, EnemiesPlugin))
            .add_systems(OnEnter(AppState::Game), pause_game)
            .add_systems(Update, (toggle_game_state).run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(GameState::Inactive), push_main_menu)
            .add_systems(
                OnExit(AppState::Game),
                (deactivate_game).run_if(not(in_state(GameState::Inactive))),
            );
    }
}
