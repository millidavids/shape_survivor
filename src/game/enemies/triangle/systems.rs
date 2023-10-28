use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::{
    components::{AnimationIndices, AnimationTimer, Health},
    enemies::{
        components::{Enemy, HordeMover},
        ENEMY_STD_AVOIDANCE, ENEMY_STD_SIZE, ENEMY_STD_SPEED,
    },
    player::components::Player,
};

use super::components::Triangle;

pub fn spawn_triangles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if enemy_query.iter().count() > 5 {
        return;
    }

    let texture_handle = asset_server.load("sprites/triangle_enemy_4_frame_64x64.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices {
        first: 0,
        last: 3,
        reverse: false,
    };

    if let Ok(window) = window_query.get_single() {
        if let Ok(player_transform) = player_query.get_single() {
            for _ in 0..10 {
                let angle = rand::random::<f32>() * PI * 2.0;
                let (y, x) = angle.sin_cos();
                commands.spawn((
                    Triangle {},
                    Name::from("Triangle"),
                    Enemy {},
                    Health(100.0),
                    HordeMover::default(),
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(animation_indices.clone().first),
                        transform: Transform::from_xyz(
                            player_transform.translation.x + x * window.width() / 2.0,
                            player_transform.translation.y + y * window.width() / 2.0,
                            100.0,
                        )
                        .with_scale(Vec3 {
                            x: 0.5,
                            y: 0.5,
                            z: 1.0,
                        }),
                        ..default()
                    },
                    animation_indices,
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                ));
            }
        }
    }
}

pub fn despawn_triangles(player_query: Query<Entity, With<Triangle>>, mut commands: Commands) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn animate_triangle(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Triangle>,
    >,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = indices.tick(&sprite.index);
        }
    }
}

pub fn direction_to_player(
    mut triangle_query: Query<(&Transform, &mut HordeMover), (With<Triangle>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Triangle>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (transform, mut hordemover) in &mut triangle_query {
            hordemover.dxdy += transform.translation - player_transform.translation;
        }
    }
}

pub fn avoid_other_triangles(
    mut triangle_query: Query<(&Transform, &mut HordeMover), (With<Triangle>, Without<Player>)>,
) {
    let mut combinations = triangle_query.iter_combinations_mut();
    while let Some([(transform_a, mut hordemover_a), (transform_b, mut hordemover_b)]) =
        combinations.fetch_next()
    {
        let weight =
            (ENEMY_STD_SIZE * 1.5) - transform_a.translation.distance(transform_b.translation);
        if weight > 0.0 {
            hordemover_a.dxdy -=
                (transform_a.translation - transform_b.translation) * weight * ENEMY_STD_AVOIDANCE;
            hordemover_b.dxdy -=
                (transform_b.translation - transform_a.translation) * weight * ENEMY_STD_AVOIDANCE;
        }
    }
}

pub fn move_triangle(
    mut triangle_query: Query<(&mut Transform, &mut HordeMover), (With<Triangle>, Without<Player>)>,
    time: Res<Time>,
) {
    for (mut transform, mut hordemover) in &mut triangle_query {
        transform.translation -=
            hordemover.dxdy.normalize_or_zero() * time.delta_seconds() * ENEMY_STD_SPEED;
        hordemover.noise();
    }
}
