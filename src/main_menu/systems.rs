use bevy::{app::AppExit, prelude::*};

use crate::states::AppState;

use super::{
    components::{MainMenu, MenuButton},
    styles::{
        get_button_text, HOVERED_BUTTON_COLOR, MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR,
        NORMAL_BUTTON_STYLE, PRESSED_BUTTON_COLOR,
    },
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // ---- Play Button ----
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        ..default()
                    },
                    MenuButton::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: get_button_text(&asset_server, "Play"),
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
                    MenuButton::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: get_button_text(&asset_server, "Quit"),
                        ..default()
                    });
                });
        });
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
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
        (&Interaction, &mut BackgroundColor, &MenuButton),
        Changed<Interaction>,
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    for (interaction, mut background_color, menu_button_option) in button_query.iter_mut() {
        match (*interaction, menu_button_option) {
            (Interaction::Pressed, MenuButton::Play) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Game);
            }
            (Interaction::Pressed, MenuButton::Quit) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            (Interaction::Hovered, _) => *background_color = HOVERED_BUTTON_COLOR.into(),
            _ => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
