use bevy::prelude::*;

use crate::game::{components::Health, player::events::AddPlayerXpEvent};

use super::{components::Enemy, events::EnemyDeathEvent};

pub fn check_health(
    enemies_query: Query<(Entity, &Health, &Enemy)>,
    mut enemy_death_event_writer: EventWriter<EnemyDeathEvent>,
    mut add_player_xp_event_writer: EventWriter<AddPlayerXpEvent>,
) {
    for (entity, health, enemy) in &enemies_query {
        if health.0 <= 0.0 {
            enemy_death_event_writer.send(EnemyDeathEvent(entity));
            add_player_xp_event_writer.send(AddPlayerXpEvent(enemy.xp));
        }
    }
}

pub fn enemy_killed(
    mut commands: Commands,
    mut enemy_death_event_reader: EventReader<EnemyDeathEvent>,
) {
    for ev in enemy_death_event_reader.iter() {
        commands.entity(ev.0).despawn_recursive();
    }
}