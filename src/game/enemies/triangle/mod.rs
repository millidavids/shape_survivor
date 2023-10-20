mod components;
mod systems;

use bevy::prelude::*;

use crate::{game::states::GameState, states::AppState};

use self::systems::{
    animate_triangle, avoid_other_triangles, despawn_triangles, direction_to_player, move_triangle,
    spawn_triangles,
};

pub struct TrianglePlugin;

impl Plugin for TrianglePlugin {
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
