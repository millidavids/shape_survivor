use bevy::prelude::*;

/// Represents the current state of the game application.
///
/// This enum is used to determine and manage the different stages or scenes
/// of the game, such as whether the game is currently showing the main menu
/// or is in the gameplay state.
///
/// # Variants
///
/// - `MainMenu`: Represents the main menu state. This is the default state when the game starts.
/// - `Game`: Represents the gameplay state where the main game logic is active.
///
/// # Usage
///
/// This component can be used with Bevy's `State` system to transition between
/// different game states, handle game state-specific logic, and more.
///
/// # Examples
///
/// ```rust
/// use bevy::prelude::*;
///
/// fn switch_to_gameplay(mut state: ResMut<State<AppState>>) {
///     state.set(AppState::Game).unwrap();
/// }
/// ```
#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
