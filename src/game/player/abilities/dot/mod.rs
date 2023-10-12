mod components;
mod systems;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::game::states::GameState;

use self::systems::{move_dots, spawn_dot, enemy_impact};

/// Bevy plugin for dot projectiles and abilities.
///
/// This plugin adds systems and components related to dot projectiles and abilities in the game.
/// It includes systems for moving dots, handling enemy impacts, and spawning new dots.
///
/// # Systems
///
/// The `DotPlugin` includes the following systems:
///
/// - `move_dots`: Moves dot projectiles based on their speed and direction.
/// - `enemy_impact`: Handles the impact of dot projectiles on enemies.
/// - `spawn_dot`: Spawns new dot projectiles periodically.
///
/// # See Also
///
/// - [Bevy](https://bevyengine.org/): Bevy game engine's official website.
/// - [`Plugin`](https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html): Bevy trait for defining plugins.
/// - [`move_dots`](fn.move_dots.html): A Bevy system for moving dot projectiles.
/// - [`enemy_impact`](fn.enemy_impact.html): A Bevy system for handling the impact of dot projectiles on enemies.
/// - [`spawn_dot`](fn.spawn_dot.html): A Bevy system for spawning new dot projectiles.
pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_dots,
                enemy_impact,
                spawn_dot.run_if(on_timer(Duration::from_millis(3000))),
            )
                .run_if(in_state(GameState::Running)),
        );
    }
}
