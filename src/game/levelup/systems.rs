use bevy::prelude::*;

use crate::game::states::GameState;

use super::components::{LevelUpMenu, LevelUpMenuButton};
use super::styles::{
    get_button_text, HOVERED_BUTTON_COLOR, LEVEL_UP_BUTTON_STYLE, LEVEL_UP_MENU_BACKGROUND_COLOR, LEVEL_UP_MENU_STYLE, LEVEL_UP_MENU_TRANSFORM, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR
};

pub fn spawn_level_up_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            Name::from("Level Up Menu Container"),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: LEVEL_UP_MENU_STYLE,
                    background_color: LEVEL_UP_MENU_BACKGROUND_COLOR.into(),
                    ..default()
                },
                LevelUpMenu {},
                Name::from("Level Up Menu"),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: get_button_text(&asset_server, "Level Up!"),
                    ..default()
                });

                spawn_level_up_button(parent, &asset_server, "Upgrade 1", LevelUpMenuButton::Upgrade1);
                spawn_level_up_button(parent, &asset_server, "Upgrade 2", LevelUpMenuButton::Upgrade2);
                spawn_level_up_button(parent, &asset_server, "Upgrade 3", LevelUpMenuButton::Upgrade3);

                spawn_level_up_button(parent, &asset_server, "Continue", LevelUpMenuButton::Continue);
            });
        });
}

fn spawn_level_up_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    button_type: LevelUpMenuButton,
) {
    parent
        .spawn((
            ButtonBundle {
                style: LEVEL_UP_BUTTON_STYLE,
                transform: LEVEL_UP_MENU_TRANSFORM,
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: get_button_text(asset_server, text),
                ..default()
            });
        });
}

pub fn despawn_level_up_menu(
    mut commands: Commands,
    level_up_menu_query: Query<Entity, With<LevelUpMenu>>,
) {
    if let Ok(level_up_menu_entity) = level_up_menu_query.get_single() {
        commands.entity(level_up_menu_entity).despawn_recursive();
    }
}

pub fn button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &LevelUpMenuButton),
        Changed<Interaction>,
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, level_up_button) in button_query.iter_mut() {
        match (*interaction, level_up_button) {
            (Interaction::Pressed, LevelUpMenuButton::Continue) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_game_state.set(GameState::Running);
            }
            (Interaction::Pressed, LevelUpMenuButton::Upgrade1) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                // Apply upgrade 1
            }
            (Interaction::Pressed, LevelUpMenuButton::Upgrade2) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                // Apply upgrade 2
            }
            (Interaction::Pressed, LevelUpMenuButton::Upgrade3) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                // Apply upgrade 3
            }
            (Interaction::Hovered, _) => *background_color = HOVERED_BUTTON_COLOR.into(),
            _ => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
