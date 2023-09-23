use bevy::prelude::*;

use crate::game::states::GameState;

use super::styles::{MAIN_MENU_STYLE, NORMAL_BUTTON_STYLE, get_button_text, PRESSED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR};
use super::components::{PauseMenu, PauseMenuButton};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            // ---- Play Button ----
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        ..default()
                    },
                    PauseMenuButton::Running,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: get_button_text(&asset_server, "Resume"),
                        ..default()
                    });
                });
            // ---- Quit Button ----
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
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

pub fn despawn_pause_menu(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

/// System determining button behavior
/// 
/// # Arguments
/// 
/// * `commands` - List of commands for interacting with the `World`.
/// * `next_app_state` - Mutable next app state resource.
/// * `app_exit_event_writer` - Mutable app exit event writer.
pub fn button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &PauseMenuButton),
        Changed<Interaction>,
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, pause_button_option) in button_query.iter_mut() {
        match (*interaction, pause_button_option) {
            (Interaction::Pressed, PauseMenuButton::Running) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_game_state.set(GameState::Running);
            }
            (Interaction::Pressed, PauseMenuButton::MainMenu) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_game_state.set(GameState::Inactive);
            }
            (Interaction::Hovered, _) => *background_color = HOVERED_BUTTON_COLOR.into(),
            _ => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}