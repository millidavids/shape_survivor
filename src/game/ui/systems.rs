use bevy::prelude::*;

use crate::game::player::{components::Player, events::PlayerLevelUpEvent};

use super::{components::{UI, XPBar}, styles::XP_BAR_STYLE};

pub fn spawn_xp_bar(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: XP_BAR_STYLE,
            background_color: Color::RED.into(),
            ..default() 
        }, 
        UI {},
        XPBar {},
    ));
}

pub fn animate_xp_bar(
    mut xp_bar_query: Query<&mut Style, With<XPBar>>,
    player_query: Query<&Player>,
    mut player_level_up_event_reader: EventReader<PlayerLevelUpEvent>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut xp_bar_style) = xp_bar_query.get_single_mut() {
            if player_level_up_event_reader.iter().count() > 0 {
                xp_bar_style.width = Val::Percent(0.0);
            } else {
                let xp_percent = player.xp.0 / player.xp.1 * 100.0;
                match xp_bar_style.width {
                    Val::Percent(value) => {
                        if value < xp_percent {
                            xp_bar_style.width = Val::Percent(value + 0.5 + (1.0 - (value / xp_percent)));
                        }
                    },
                    _ => {},
                }
            }
        }
    }
}
