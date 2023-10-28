use bevy::prelude::*;

use crate::game::player::components::Player;
use super::{components::Drop, DROPS_DISTANCE_THRESHOLD};

pub fn move_drops(
    mut drops_query: Query<&mut Transform, (With<Drop>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Drop>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut transform in &mut drops_query {
            let direction = player_transform.translation - transform.translation;
            let distance = player_transform.translation.distance(transform.translation);

            if distance < DROPS_DISTANCE_THRESHOLD {
                transform.translation += direction.normalize_or_zero() * (DROPS_DISTANCE_THRESHOLD - distance) * 2.0 * time.delta_seconds();
            }
        }
    }
}