use bevy::{prelude::*, window::PrimaryWindow};

use super::states::AppState;

/// Spawns a 2D camera centered in the primary window.
///
/// This function creates and spawns a 2D camera at the center of the primary window.
/// The camera's position is determined based on the width and height of the primary window.
///
/// # Parameters
/// - `commands`: A mutable reference to the Bevy command buffer. It allows for queueing up
///   changes to the world that will be applied at the end of the current stage.
/// - `window_query`: A query to access the primary window's properties. This is used to
///   get the width and height of the window to position the camera.
///
/// # Panics
/// This function will panic if there's no primary window available from the `window_query`.
///
/// # Examples
///
/// Assuming you're in a Bevy system function:
///
/// ```
/// fn my_system(
///     mut commands: Commands,
///     window_query: Query<&Window, With<PrimaryWindow>>,
/// ) {
///     spawn_camera(commands, window_query);
/// }
/// ```
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

/// Toggles the application state between `MainMenu` and `Game` when the 'M' key is pressed.
///
/// This function checks for the just pressed 'M' key and toggles the application's state
/// between the `MainMenu` and `Game` states. If the current state is `MainMenu`, it will
/// transition to `Game` and vice versa.
///
/// # Parameters
/// - `keyboard_input`: A resource representing the state of the keyboard input.
/// - `current_app_state`: A resource representing the current application state.
/// - `next_app_state`: A mutable resource used to set the next application state.
///
/// # Examples
///
/// To use this function inside a Bevy system:
///
/// ```rust
/// fn my_system(
///     keyboard_input: Res<Input<KeyCode>>,
///     current_app_state: Res<State<AppState>>,
///     mut next_app_state: ResMut<NextState<AppState>>,
/// ) {
///     toggle_app_state(keyboard_input, current_app_state, next_app_state);
/// }
/// ```
pub fn toggle_app_state(
    keyboard_input: Res<Input<KeyCode>>,
    current_app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        match *current_app_state.get() {
            AppState::MainMenu => next_app_state.set(AppState::Game),
            AppState::Game => next_app_state.set(AppState::MainMenu),
        }
    }
}

/// Sets the application's next state to `MainMenu`.
///
/// This function updates the application's next state to be the `MainMenu` state.
///
/// # Parameters
/// - `next_app_state`: A mutable resource used to set the next application state.
///
/// # Examples
///
/// In the context of a Bevy system or another function:
///
/// ```rust
/// fn my_system(mut next_app_state: ResMut<NextState<AppState>>) {
///     push_main_menu(next_app_state);
/// }
/// ```
pub fn push_main_menu(mut next_app_state: ResMut<NextState<AppState>>) {
    next_app_state.set(AppState::MainMenu);
}
