pub mod components;
mod systems;

use bevy::prelude::*;

use crate::{game::states::GameState, states::AppState};

use self::systems::{
    check_bounds, despawn_dots, enemy_impact, has_dot_mod, move_dots, spawn_dot,
    spawn_dot_condition,
};

pub const DEFAULT_DOT_RADIUS: f32 = 2.0;

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_dots,
                enemy_impact,
                check_bounds,
                spawn_dot.run_if(spawn_dot_condition),
            )
                .run_if(in_state(GameState::Running))
                .run_if(has_dot_mod),
        )
        .add_systems(OnExit(AppState::Game), despawn_dots);
    }
}
