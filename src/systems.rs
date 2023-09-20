use bevy::{prelude::*, window::PrimaryWindow};

use crate::states::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn toggle_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    current_game_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        match *current_game_state.get() {
            AppState::MainMenu => next_app_state.set(AppState::Game),
            AppState::Game => next_app_state.set(AppState::MainMenu),
        }
    }
}
