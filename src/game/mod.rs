mod components;
mod paused;
mod resources;
mod states;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::{states::AppState, systems::push_main_menu};

use self::{
    paused::PausedPlugin,
    states::GameState,
    systems::{deactivate_game, pause_game, toggle_game_state},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(PausedPlugin)
            .add_systems(OnEnter(AppState::Game), pause_game)
            .add_systems(Update, (toggle_game_state).run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(GameState::Inactive), push_main_menu)
            .add_systems(
                OnExit(AppState::Game),
                (deactivate_game).run_if(not(in_state(GameState::Inactive))),
            );
    }
}
