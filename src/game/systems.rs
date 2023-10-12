use bevy::prelude::*;

use super::states::GameState;

/// Toggles the game's state between `Running`, `Paused`, and `Inactive` when the Space key is pressed.
///
/// This system checks if the Space key has just been pressed. Depending on the current game state, it transitions 
/// to a new state:
///
/// - From `Running` to `Paused`.
/// - From `Paused` to `Running`.
/// - From `Inactive` to `Paused`.
///
/// The system provides a way to quickly pause or resume the game and is especially useful for providing 
/// players with a way to temporarily halt gameplay.
///
/// # Parameters
///
/// - `keyboard_input`: A resource representing the state of the keyboard input.
/// - `current_game_state`: A resource representing the current game state.
/// - `mut next_game_state`: A mutable resource used to set the next game state.
///
/// # Examples
///
/// To use this system inside a Bevy app:
///
/// ```rust
/// App::new()
///     .add_systems(Update, toggle_game_state)
///     .run();
/// ```
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

/// Sets the game's state to `Paused`.
///
/// This system immediately transitions the game to the `Paused` state, effectively 
/// pausing all in-game activities that respect this state.
///
/// # Parameters
///
/// - `mut next_game_state`: A mutable resource used to set the next game state.
///
/// # Examples
///
/// To use this system inside a Bevy app:
///
/// ```rust
/// App::new()
///     .add_systems(Update, pause_game)
///     .run();
/// ```
pub fn pause_game(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Paused);
}

/// Sets the game's state to `Inactive`.
///
/// This system transitions the game to the `Inactive` state, halting any in-game activities
/// that check or respond to this specific state. Useful for situations where gameplay 
/// should not only be paused, but also be transitioned to a non-active state, such as 
/// when transitioning to a main menu or loading screen.
///
/// # Parameters
///
/// - `mut next_game_state`: A mutable resource used to set the next game state.
///
/// # Examples
///
/// To use this system inside a Bevy app:
///
/// ```rust
/// App::new()
///     .add_systems(Update, deactivate_game)
///     .run();
/// ```
pub fn deactivate_game(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Inactive);
}