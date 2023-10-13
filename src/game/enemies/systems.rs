use bevy::prelude::*;

use crate::game::components::Health;

use super::components::Enemy;

pub fn check_health(mut commands: Commands, enemies_query: Query<(Entity, &Health), With<Enemy>>) {
    for (entity, health) in &enemies_query {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
