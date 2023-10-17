mod components;
mod systems;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{game::states::GameState, states::AppState};

use self::systems::{check_bounds, despawn_dots, enemy_impact, move_dots, spawn_dot};

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
                spawn_dot.run_if(on_timer(Duration::from_millis(1000))),
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(OnExit(AppState::Game), despawn_dots);
    }
}
