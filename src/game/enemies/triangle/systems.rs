use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::{
    components::{AnimationIndices, AnimationTimer},
    enemies::components::Enemy,
    player::components::Player,
};

use super::{components::Triangle, TRIANGLE_SPEED};

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
                    Enemy {},
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

pub fn move_triangle(
    mut triangle_query: Query<&mut Transform, (With<Triangle>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Triangle>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut triangle_transform in triangle_query.iter_mut() {
            let mut direction = triangle_transform.translation - player_transform.translation;

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            triangle_transform.translation -= direction * TRIANGLE_SPEED * time.delta_seconds();
        }
    }
}
