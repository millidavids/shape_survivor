use bevy::prelude::*;

use crate::game::{
    components::Health, drops::experience::events::ExperienceSpawnEvent,
    player::abilities::events::TransmitDamage,
};

use super::components::Enemy;

pub fn check_health(
    mut commands: Commands,
    enemies_query: Query<(Entity, &Health, &Transform)>,
    mut experience_spawn_event_writer: EventWriter<ExperienceSpawnEvent>,
) {
    for (entity, health, transform) in &enemies_query {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
            experience_spawn_event_writer.send(ExperienceSpawnEvent(*transform))
        }
    }
}

pub fn damage_enemies(
    mut transmit_damage_event_reader: EventReader<TransmitDamage>,
    mut enemies_query: Query<&mut Health, With<Enemy>>,
) {
    for event in &mut transmit_damage_event_reader {
        if let Ok(mut health) = enemies_query.get_mut(event.target) {
            health.0 -= event.damage;
        }
    }
}
