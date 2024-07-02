use bevy::prelude::*;

use crate::game::{
    components::Health, drops::experience::events::ExperienceSpawnEvent,
    player::abilities::events::TransmitDamage,
    grid::{GRID_WIDTH, GRID_HEIGHT},
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

pub fn update_enemy_targetable(
    mut enemies_query: Query<(&Transform, &mut Enemy)>,
) {
    for (transform, mut enemy) in &mut enemies_query {
        let x = transform.translation.x;
        let y = transform.translation.y;

        if x >= 0.0 && x <= GRID_WIDTH as f32 && y >= 0.0 && y <= GRID_HEIGHT as f32 {
            enemy.targetable = true;
        } else {
            enemy.targetable = false;
        }
    }
}
