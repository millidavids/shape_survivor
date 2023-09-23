use bevy::prelude::*;

use super::{states::GameState, components::Game};

pub fn toggle_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match *current_game_state.get() {
            GameState::Running => next_game_state.set(GameState::Paused),
            GameState::Paused => next_game_state.set(GameState::Running),
            GameState::Inactive => next_game_state.set(GameState::Paused),
        }
    }
}

pub fn pause_game(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Paused);
}

pub fn deactivate_game(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Inactive);
}

pub fn spawn_game(
    mut commands: Commands,
) {
    commands.spawn(Game {});
}