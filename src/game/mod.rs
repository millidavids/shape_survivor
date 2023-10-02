mod components;
mod enemies;
mod grid;
mod paused;
mod player;
mod states;
mod systems;

use bevy::prelude::*;

use crate::{states::AppState, systems::push_main_menu};

use self::{
    grid::GridPlugin,
    paused::PausedPlugin,
    player::PlayerPlugin,
    states::GameState,
    systems::{deactivate_game, pause_game, toggle_game_state}, enemies::EnemiesPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((PausedPlugin, PlayerPlugin, GridPlugin, EnemiesPlugin))
            .add_systems(OnEnter(AppState::Game), pause_game)
            .add_systems(Update, (toggle_game_state).run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(GameState::Inactive), push_main_menu)
            .add_systems(
                OnExit(AppState::Game),
                (deactivate_game).run_if(not(in_state(GameState::Inactive))),
            );
    }
}
