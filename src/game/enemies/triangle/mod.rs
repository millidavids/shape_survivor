mod components;
mod systems;

use bevy::prelude::*;

use crate::{game::states::GameState, states::AppState};

use self::systems::{animate_triangle, despawn_triangles, move_triangle, spawn_triangles};

pub const TRIANGLE_SPEED: f32 = 125.0;

pub struct TrianglePlugin;

impl Plugin for TrianglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_triangle, move_triangle, spawn_triangles).run_if(in_state(GameState::Running)),
        )
        .add_systems(OnExit(AppState::Game), despawn_triangles);
    }
}
