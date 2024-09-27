use bevy::prelude::*;

use super::{states::GameState, components::{AnimationIndices, AnimationTimer}, player::events::PlayerLevelUpEvent};

pub fn toggle_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        match *current_game_state.get() {
            GameState::Running | GameState::Inactive => next_game_state.set(GameState::Paused),
            _ => next_game_state.set(GameState::Running),
        }
    }
}

pub fn new_game(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::NewGame);
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

pub fn handle_player_level_up(
    mut player_level_up_event_reader: EventReader<PlayerLevelUpEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for _event in player_level_up_event_reader.iter() {
        next_game_state.set(GameState::LevelUp);
    }
}