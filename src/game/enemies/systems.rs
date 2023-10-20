use bevy::prelude::*;

use crate::game::{components::Health, player::abilities::events::TransmitDamage};

use super::{components::Enemy, events::EnemyDeathEvent};

pub fn check_health(
    mut commands: Commands,
    enemies_query: Query<(Entity, &Health, &Enemy)>,
    mut enemy_death_event_writer: EventWriter<EnemyDeathEvent>,
) {
    for (entity, health, enemy) in &enemies_query {
        if health.0 <= 0.0 {
            enemy_death_event_writer.send(EnemyDeathEvent(enemy.xp));
            commands.entity(entity).despawn_recursive();
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