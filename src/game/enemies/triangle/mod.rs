mod components;
mod systems;

use bevy::prelude::*;

use crate::{states::AppState, game::states::GameState};

use self::systems::{animate_triangle, despawn_triangle, spawn_triangle};

pub struct TrianglePlugin;

impl Plugin for TrianglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_triangle)
            .add_systems(Update, (animate_triangle).run_if(in_state(GameState::Running)))
            .add_systems(OnExit(AppState::Game), despawn_triangle);
    }
}
