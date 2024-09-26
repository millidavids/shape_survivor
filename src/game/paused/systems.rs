use bevy::prelude::*;

use crate::game::{
    states::GameState,
    resources::GameStatus,
};
use crate::states::AppState;

use super::components::{PauseMenu, PauseMenuButton};
use super::styles::{
    get_button_text, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PAUSE_BUTTON_STYLE,
    PAUSE_MENU_STYLE, PAUSE_MENU_TRANSFORM, PRESSED_BUTTON_COLOR,
};

pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_status: Res<GameStatus>,
) {
    let resume_or_start_text = if game_status.new_game_requested {
        "Start"
    } else {
        "Resume"
    };

    commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                transform: PAUSE_MENU_TRANSFORM,
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            // Resume/Start Button
            parent
                .spawn((
                    ButtonBundle {
                        style: PAUSE_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    if game_status.new_game_requested {
                        PauseMenuButton::Start
                    } else {
                        PauseMenuButton::Resume
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: get_button_text(&asset_server, resume_or_start_text),
                        ..default()
                    });
                });

            // Main Menu Button
            parent
                .spawn((
                    ButtonBundle {
                        style: PAUSE_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PauseMenuButton::MainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: get_button_text(&asset_server, "Main Menu"),
                        ..default()
                    });
                });
        });
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

pub fn button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &PauseMenuButton),
        Changed<Interaction>,
    >,
    mut game_status: ResMut<GameStatus>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color, menu_button) in button_query.iter_mut() {
        match (*interaction, menu_button) {
            (Interaction::Pressed, PauseMenuButton::Resume) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_game_state.set(GameState::Running);
                game_status.game_paused = false;
            }
            (Interaction::Pressed, PauseMenuButton::Start) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_game_state.set(GameState::Running);
                game_status.new_game_requested = false;
                game_status.game_paused = false;
                // Add any additional logic needed to start a new game
            }
            (Interaction::Pressed, PauseMenuButton::MainMenu) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::MainMenu);
                game_status.new_game_requested = false;
                game_status.game_paused = false;
            }
            (Interaction::Hovered, _) => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            (Interaction::None, _) => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
