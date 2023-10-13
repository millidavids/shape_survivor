pub mod components;
mod systems;
mod triangle;

use bevy::prelude::*;

use self::{systems::check_health, triangle::TrianglePlugin};

use super::states::GameState;

pub const ENEMY_STD_SPEED: f32 = 125.0;
pub const ENEMY_STD_AVOIDANCE: f32 = 0.2;
pub const ENEMY_STD_SIZE: f32 = 32.0;

/// Bevy plugin for managing enemy entities and behaviors in the game.
///
/// The `EnemiesPlugin` is responsible for adding and configuring systems and resources related to enemy entities
/// and their behaviors. This plugin can include systems for spawning enemies, updating their AI, handling collisions,
/// and more.
///
/// To use this plugin, simply add it to your Bevy app using the `add_plugins` method during the application setup.
/// It will automatically integrate the necessary systems and resources to handle enemies in your game.
///
/// # Examples:
/// ```rust
/// # use bevy::prelude::*;
///
/// fn main() {
///     App::build()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(EnemiesPlugin)
///         .add_system(StartUp, setup_game)
///         .run();
/// }
/// ```
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    /// Builds the `EnemiesPlugin`, adding related systems and resources to the Bevy app.
    ///
    /// This function is called automatically when adding the `EnemiesPlugin` to your Bevy app using
    /// the `add_plugin` method.
    ///
    /// # Parameters
    /// - `app`: A mutable reference to the Bevy app to which the plugin should add its systems and resources.
    fn build(&self, app: &mut App) {
        // Add systems and resources related to enemy entities and behaviors here
        app.add_plugins(TrianglePlugin)
            .add_systems(Update, check_health.run_if(in_state(GameState::Running)));
    }
}
