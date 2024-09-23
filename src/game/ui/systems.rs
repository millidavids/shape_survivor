use bevy::prelude::*;

use crate::game::player::{components::Player, events::PlayerLevelUpEvent};

use super::{components::{XPBar, XPBarContainer, XPText, UI}, styles::{get_xp_text, UI_STYLE, XP_BAR_CONTAINER_STYLE, XP_BAR_STYLE, XP_TEXT_STYLE}};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: UI_STYLE,
            ..default()
        },
        UI {},
    ))
    .with_children(|parent| {
        spawn_xp_bar(parent, &asset_server);
    });
}

fn spawn_xp_bar(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: XP_BAR_CONTAINER_STYLE,
            background_color: Color::hsla(0.0, 0.0, 1.0, 0.5).into(),
            border_color: Color::BLACK.into(),
            ..default()
        },
        XPBarContainer {},
    ))
    .with_children(|container| {
        spawn_xp_text(container, asset_server);
        container.spawn((
            NodeBundle {
                style: XP_BAR_STYLE,
                background_color: Color::CYAN.into(),
                ..default()
            },
            XPBar {},
        ));
    });
}

fn spawn_xp_text(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        TextBundle {
            text: get_xp_text(asset_server, "LV. 0"),
            style: XP_TEXT_STYLE,
            ..default()
        },
        XPText {},
    ));
}

pub fn animate_xp_bar(
    mut xp_bar_query: Query<&mut Style, With<XPBar>>,
    player_query: Query<&Player>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut xp_bar_style) = xp_bar_query.get_single_mut() {
            let xp_percent = player.xp.0 / player.xp.1 * 100.0;
            xp_bar_style.width = Val::Percent(xp_percent);
        }
    }
}

pub fn update_xp_text (
    mut xp_text_query: Query<&mut Text, With<XPText>>,
    mut player_level_up_event_reader: EventReader<PlayerLevelUpEvent>,
) {
    if let Ok(mut xp_text) = xp_text_query.get_single_mut() {
        for event in player_level_up_event_reader.iter() {
            xp_text.sections[0].value = format!("LV. {}", event.0);
        }
    }
}