use bevy::prelude::*;

use super::{components::{AnimationIndices, AnimationTimer}, resources::GameStatus, states::GameState};

pub fn toggle_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_status: ResMut<GameStatus>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        match *current_game_state.get() {
            GameState::Running => {
                next_game_state.set(GameState::Paused);
                game_status.game_paused = true;
            },
            GameState::Paused => {
                next_game_state.set(GameState::Running);
                game_status.game_paused = false;
                game_status.new_game_requested = false;
            },
            GameState::Inactive => {
                next_game_state.set(GameState::Paused);
                game_status.new_game_requested = true;
                game_status.game_paused = true;
            },
        }
    }
}

pub fn pause_game(mut next_game_state: ResMut<NextState<GameState>>, mut game_status: ResMut<GameStatus>) {
    next_game_state.set(GameState::Paused);
    game_status.game_paused = true;
}

pub fn deactivate_game(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Inactive);
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        )>,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = indices.tick(&sprite.index);
        }
    }
}