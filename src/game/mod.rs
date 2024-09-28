mod components;
mod drops;
mod enemies;
mod grid;
mod paused;
mod player;
mod levelup;
mod states;
mod systems;
mod ui;

use bevy::prelude::*;

use crate::{states::AppState, systems::push_main_menu};

use self::{
    drops::DropsPlugin,
    enemies::EnemiesPlugin,
    grid::GridPlugin,
    levelup::LevelUpPlugin,
    paused::PausedPlugin,
    player::PlayerPlugin,
    states::GameState,
    systems::{animate_sprites, deactivate_game, new_game, toggle_game_state, handle_player_level_up}, ui::UIPlugin,
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
                UIPlugin,
                LevelUpPlugin,
            ))
            .add_systems(OnEnter(AppState::Game), new_game)
            .add_systems(
                Update,
                (
                    toggle_game_state.run_if(in_state(AppState::Game)),
                    (animate_sprites, handle_player_level_up).run_if(in_state(GameState::Running)),
                ),
            )
            .add_systems(OnEnter(GameState::Inactive), push_main_menu)
            .add_systems(
                OnExit(AppState::Game),
                (deactivate_game).run_if(not(in_state(GameState::Inactive))),
            );
    }
}
