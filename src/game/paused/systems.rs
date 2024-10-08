use bevy::prelude::*;

use crate::game::states::GameState;

use super::components::{PauseMenu, PauseMenuButton};
use super::styles::{
    get_button_text, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PAUSE_BUTTON_STYLE,
    PAUSE_MENU_STYLE, PAUSE_MENU_TRANSFORM, PRESSED_BUTTON_COLOR,
    PAUSE_MENU_BACKGROUND_COLOR,
};

pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
) {
    let resume_start_text = match game_state.get() {
        GameState::NewGame => "Start",
        _ => "Resume",
    };
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Name::from("Pause Menu Container"),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: PAUSE_MENU_STYLE,
                    background_color: PAUSE_MENU_BACKGROUND_COLOR.into(),
                    ..default()
                },
                PauseMenu {},
                Name::from("Pause Menu"),
            ))
            .with_children(|parent| {
                // ---- Play Button ----
                parent
                    .spawn((
                        ButtonBundle {
                            style: PAUSE_BUTTON_STYLE,
                            transform: PAUSE_MENU_TRANSFORM,
                            ..default()
                        },
                        PauseMenuButton::Running,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: get_button_text(&asset_server, resume_start_text),
                            ..default()
                        });
                    });
                // ---- Quit Button ----
                parent
                    .spawn((
                        ButtonBundle {
                            style: PAUSE_BUTTON_STYLE,
                            transform: PAUSE_MENU_TRANSFORM,
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
