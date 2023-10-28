mod components;
mod drops;
mod enemies;
mod grid;
mod paused;
mod player;
mod states;
mod systems;

use bevy::prelude::*;

use crate::{states::AppState, systems::push_main_menu};

use self::{
    drops::DropsPlugin,
    enemies::EnemiesPlugin,
    grid::GridPlugin,
    paused::PausedPlugin,
    player::PlayerPlugin,
    states::GameState,
    systems::{animate_sprites, deactivate_game, pause_game, toggle_game_state},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((
                PausedPlugin,
                PlayerPlugin,
                GridPlugin,
                EnemiesPlugin,
                DropsPlugin,
            ))
            .add_systems(OnEnter(AppState::Game), pause_game)
            .add_systems(
                Update,
                (
                    toggle_game_state.run_if(in_state(AppState::Game)),
                    animate_sprites.run_if(in_state(GameState::Running)),
                ),
            )
            .add_systems(OnEnter(GameState::Inactive), push_main_menu)
            .add_systems(
                OnExit(AppState::Game),
                (deactivate_game).run_if(not(in_state(GameState::Inactive))),
            );
    }
}
