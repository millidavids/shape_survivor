use bevy::prelude::*;

/// Represents the various states of the game.
///
/// `GameState` provides different states that the game can be in at any given moment. This
/// allows for easier management and transitions between different gameplay stages or modes.
///
/// Variants:
/// - `Inactive`: Represents a state where the game is not actively running. This could be
///   used for scenarios like the game's main menu or loading screens.
/// - `Paused`: Represents a state where the game's progress or activities are temporarily halted,
///   but can be resumed from the point of pausing.
/// - `Running`: Represents the normal, active state where gameplay activities are ongoing.
///
/// By default, the game state is set to `Inactive`.
///
/// # Examples
///
/// Using `GameState` in a Bevy system to check the current state:
///
/// ```rust
/// fn my_system(game_state: Res<State<GameState>>) {
///     if *game_state.current() == GameState::Running {
///         // Perform gameplay-related logic here
///     }
/// }
/// ```
///
/// Switching between game states:
///
/// ```rust
/// fn another_system(mut next_game_state: ResMut<NextState<GameState>>) {
///     // Switch to the Running state
///     next_game_state.set(GameState::Running);
/// }
/// ```
#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    Inactive,
    Paused,
    Running,
}
