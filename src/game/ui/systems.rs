use bevy::prelude::*;

use crate::game::player::{components::Player, events::PlayerLevelUpEvent};

use super::{
    components::{HealthBar, HealthBarContainer, HealthBarText, XPBar, XPBarContainer, XPText, UI},
    styles::{
        get_filling_bar_text, FILLING_BAR_CONTAINER_STYLE, FILLING_BAR_CONTAINER_STYLE_BOTTOM,
        FILLING_BAR_STYLE, FILLING_BAR_TEXT_STYLE, UI_STYLE,
    },
};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: UI_STYLE,
                ..default()
            },
            UI {},
        ))
        .with_children(|parent| {
            spawn_xp_bar(parent, &asset_server);
            spawn_health_bar(parent, &asset_server);
        });
}

pub fn despawn_ui(mut commands: Commands, ui_query: Query<Entity, With<UI>>) {
    if let Ok(ui) = ui_query.get_single() {
        commands.entity(ui).despawn_recursive();
    }
}

fn spawn_xp_bar(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: FILLING_BAR_CONTAINER_STYLE_BOTTOM,
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
                    style: FILLING_BAR_STYLE,
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
            text: get_filling_bar_text(asset_server, "LV. 0"),
            style: FILLING_BAR_TEXT_STYLE,
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

pub fn update_xp_text(
    mut xp_text_query: Query<&mut Text, With<XPText>>,
    mut player_level_up_event_reader: EventReader<PlayerLevelUpEvent>,
) {
    if let Ok(mut xp_text) = xp_text_query.get_single_mut() {
        for event in player_level_up_event_reader.iter() {
            xp_text.sections[0].value = format!("LVL: {}", event.0);
        }
    }
}

fn spawn_health_bar(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: FILLING_BAR_CONTAINER_STYLE,
                background_color: Color::hsla(0.0, 0.0, 1.0, 0.5).into(),
                border_color: Color::BLACK.into(),
                ..default()
            },
            HealthBarContainer {},
        ))
        .with_children(|container| {
            spawn_health_bar_text(container, asset_server);
            container.spawn((
                NodeBundle {
                    style: FILLING_BAR_STYLE,
                    background_color: Color::RED.into(),
                    ..default()
                },
                HealthBar {},
            ));
        });
}

fn spawn_health_bar_text(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        TextBundle {
            text: get_filling_bar_text(asset_server, "HP: "),
            style: FILLING_BAR_TEXT_STYLE,
            ..default()
        },
        HealthBarText {},
    ));
}

pub fn animate_health_bar(
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
    player_query: Query<&Player>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut health_bar_style) = health_bar_query.get_single_mut() {
            let health_percent = player.health.0 / player.health.1 * 100.0;
            health_bar_style.width = Val::Percent(health_percent);
        }
    }
}
