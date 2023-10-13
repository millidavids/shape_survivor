pub mod components;
mod systems;

use bevy::prelude::*;

use crate::states::AppState;

use self::systems::{despawn_grid, spawn_grid};

pub const GRID_SMALL_BOX_LENGTH: u32 = 32;
pub const GRID_LARGE_BOX_LENGTH: u32 = GRID_SMALL_BOX_LENGTH * 5;

pub const NUM_SMALL_BOX_WIDTH: u32 = 850;
pub const NUM_SMALL_BOX_HEIGHT: u32 = 1100;
pub const NUM_LARGE_BOX_WIDTH: u32 = NUM_SMALL_BOX_WIDTH / 5;
pub const NUM_LARGE_BOX_HEIGHT: u32 = NUM_SMALL_BOX_HEIGHT / 5;

pub const GRID_WIDTH: u32 = GRID_SMALL_BOX_LENGTH * NUM_SMALL_BOX_WIDTH;
pub const GRID_HEIGHT: u32 = GRID_SMALL_BOX_LENGTH * NUM_SMALL_BOX_HEIGHT;

/// A Bevy plugin responsible for managing the visual grid in the game world.
///
/// The `GridPlugin` facilitates the spawning and despawning of the visual grid based on the game's state.
/// Specifically, when entering the `AppState::Game` state, the grid is spawned, and upon exiting this state, the grid is despawned.
///
/// # Usage:
/// To use the `GridPlugin`, simply add it to your Bevy app during the app building process.
///
/// # Examples:
/// ```rust
/// # use bevy::prelude::*;
///
/// # fn main() {
/// let mut app = App::build();
///
/// // Add the GridPlugin to your app.
/// app.add_plugins(GridPlugin);
/// # }
/// ```
///
/// # Systems:
/// - `spawn_grid`: Spawned during `OnEnter(AppState::Game)`. This system spawns the visual grid and its associated entities in the game world.
/// - `despawn_grid`: Spawned during `OnExit(AppState::Game)`. This system despawns the grid and its children entities from the game world.
///
/// # Note:
/// Ensure that the relevant systems (`spawn_grid` and `despawn_grid`) as well as the `Grid` component (if utilized by the systems) are defined in the same module or are globally accessible for this plugin to function correctly.
pub struct GridPlugin;

impl Plugin for GridPlugin {
    /// Configures the app to use the `GridPlugin`.
    ///
    /// # Parameters:
    /// * `app`: A mutable reference to the Bevy App.
    ///
    /// This function binds the `spawn_grid` system to execute when the game state enters `AppState::Game`.
    /// Additionally, it binds the `despawn_grid` system to execute when the game state exits `AppState::Game`.
    ///
    /// # Example:
    /// ```rust
    /// # use bevy::prelude::*;
    ///
    /// # fn main() {
    /// let mut app = App::build();
    ///
    /// // Configures the app to use the GridPlugin.
    /// app.add_plugins(GridPlugin);
    /// # }
    /// ```
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_grid)
            .add_systems(OnExit(AppState::Game), despawn_grid);
    }
}
