mod components;
mod systems;

use bevy::prelude::*;

use crate::{game::states::GameState, states::AppState};

use self::systems::{
    animate_triangle, despawn_triangles, direction_to_player, move_triangle, spawn_triangles, avoid_other_triangles,
};

/// Bevy plugin for triangle enemy entities and their behaviors.
///
/// The `TrianglePlugin` is responsible for managing and updating triangle enemy entities in the game.
///
/// # Overview
///
/// This plugin adds and updates triangle enemy entities and their behaviors during the game.
/// It includes systems for animation, spawning, movement, and interaction with other game entities.
///
pub struct TrianglePlugin;

impl Plugin for TrianglePlugin {
    /// Implements the Bevy `Plugin` trait for managing triangle enemy entities and their behaviors.
    ///
    /// The `build` function initializes and configures the `TrianglePlugin` for use within a Bevy app.
    ///
    /// # Parameters
    ///
    /// - `app`: A mutable reference to the Bevy app where this plugin is added.
    ///
    /// # See Also
    ///
    /// - [`Plugin`](https://bevyengine.github.io/bevy/bevy/app/trait.Plugin.html): Bevy trait for defining plugins.
    /// - [`animate_triangle`](fn.animate_triangle.html): Bevy system for animating triangle enemies.
    /// - [`spawn_triangles`](fn.spawn_triangles.html): Bevy system for spawning new triangle enemies.
    /// - [`move_triangle`](fn.move_triangle.html): Bevy system for updating triangle enemy movements.
    /// - [`despawn_triangles`](fn.despawn_triangles.html): Bevy system for despawning unnecessary triangle enemies.
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_triangle,
                spawn_triangles,
                (direction_to_player, avoid_other_triangles, move_triangle).chain(),
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(OnExit(AppState::Game), despawn_triangles);
    }
}
