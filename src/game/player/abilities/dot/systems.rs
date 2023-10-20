use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use rand::seq::IteratorRandom;

use crate::game::{
    enemies::components::Enemy,
    grid::{GRID_HEIGHT, GRID_WIDTH},
    player::{
        abilities::{
            components::{Ability, Projectile},
            events::TransmitDamage,
            DEFAULT_ABILITY_SPEED,
        },
        components::Player,
    },
};

use super::{components::Dot, DEFAULT_DOT_RADIUS};

pub fn spawn_dot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Some(random_enemy_transform) = enemy_query.iter().choose(&mut rand::thread_rng()) {
            commands.spawn((
                Dot {},
                Projectile {
                    speed: DEFAULT_ABILITY_SPEED,
                    direction: player_transform.translation - random_enemy_transform.translation,
                },
                Ability { damage: 100.0 },
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(DEFAULT_DOT_RADIUS).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    transform: Transform::from_xyz(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        player_transform.translation.z - 1.0,
                    )
                    .with_scale(Vec3 {
                        x: 2.0,
                        y: 2.0,
                        z: 1.0,
                    }),
                    ..default()
                },
            ));
        }
    }
}

pub fn despawn_dots(mut commands: Commands, dots_query: Query<Entity, With<Dot>>) {
    for entity in &dots_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn move_dots(mut dots_query: Query<(&mut Transform, &Projectile), With<Dot>>, time: Res<Time>) {
    for (mut dot_transform, projectile) in &mut dots_query {
        dot_transform.translation -=
            projectile.direction.normalize_or_zero() * projectile.speed * time.delta_seconds();
    }
}
pub fn enemy_impact(
    mut commands: Commands,
    mut enemies_query: Query<(Entity, &Transform, &Handle<TextureAtlas>), With<Enemy>>,
    dots_query: Query<(Entity, &Transform, &Ability), With<Dot>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut transmit_damage_event_writer: EventWriter<TransmitDamage>,
) {
    for (dot_entity, dot_transform, dot_ability) in &dots_query {
        for (enemy_entity, enemy_transform, enemy_texture_atlas) in &mut enemies_query {
            if collide(
                dot_transform.translation,
                Vec2::splat(DEFAULT_DOT_RADIUS * 2.0),
                enemy_transform.translation,
                Vec2::splat(texture_atlases.get(enemy_texture_atlas).unwrap().size.y / 2.0),
            ) != None
            {
                commands.entity(dot_entity).despawn_recursive();
                transmit_damage_event_writer.send(TransmitDamage {target: enemy_entity, damage: dot_ability.damage });
            }
        }
    }
}

pub fn check_bounds(mut commands: Commands, dots_query: Query<(Entity, &Transform), With<Dot>>) {
    for (entity, transform) in &dots_query {
        let x = transform.translation.x;
        let y = transform.translation.y;
        if x <= 0.0 || x >= GRID_WIDTH as f32 || y <= 0.0 || y >= GRID_HEIGHT as f32 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
