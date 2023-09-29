use bevy::prelude::*;


use super::{
    components::Grid, GRID_LARGE_BOX_LENGTH, GRID_SMALL_BOX_LENGTH,
    NUM_LARGE_BOX_HEIGHT, NUM_LARGE_BOX_WIDTH, NUM_SMALL_BOX_HEIGHT, NUM_SMALL_BOX_WIDTH, GRID_WIDTH, GRID_HEIGHT,
};

pub fn spawn_grid(mut commands: Commands) {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    GRID_WIDTH as f32 / 2.0,
                    GRID_HEIGHT as f32 / 2.0,
                    0.0,
                )),
                sprite: Sprite {
                    color: Color::hsla(1.0, 1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(
                        GRID_WIDTH as f32,
                        GRID_HEIGHT as f32,
                    )),
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
            Grid {},
        ))
        .with_children(|parent| {
            for i in 0..=NUM_SMALL_BOX_WIDTH {
                parent.spawn(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        (i as f32 - NUM_SMALL_BOX_WIDTH as f32 / 2.0) * GRID_SMALL_BOX_LENGTH as f32,
                        0.,
                        1.0,
                    )),
                    sprite: Sprite {
                        color: Color::hsla(191.0, 0.86, 0.42, 0.5),
                        custom_size: Some(Vec2::new(
                            1.0,
                            NUM_SMALL_BOX_HEIGHT as f32 * GRID_SMALL_BOX_LENGTH as f32,
                        )),
                        ..default()
                    },
                    ..default()
                });
            }
            for i in 0..=NUM_SMALL_BOX_HEIGHT {
                parent.spawn(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        0.,
                        (i as f32 - NUM_SMALL_BOX_HEIGHT as f32 / 2.0) * GRID_SMALL_BOX_LENGTH as f32,
                        1.0,
                    )),
                    sprite: Sprite {
                        color: Color::hsla(191.0, 0.86, 0.42, 0.5),
                        custom_size: Some(Vec2::new(
                            NUM_SMALL_BOX_WIDTH as f32 * GRID_SMALL_BOX_LENGTH as f32,
                            1.0,
                        )),
                        ..default()
                    },
                    ..default()
                });
            }
            for i in 0..=NUM_LARGE_BOX_WIDTH {
                parent.spawn(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        (i as f32 - NUM_LARGE_BOX_WIDTH as f32 / 2.0) * GRID_LARGE_BOX_LENGTH as f32,
                        0.,
                        1.0,
                    )),
                    sprite: Sprite {
                        color: Color::hsla(1.0, 0.86, 0.42, 0.5),
                        custom_size: Some(Vec2::new(
                            1.5,
                            NUM_LARGE_BOX_HEIGHT as f32 * GRID_LARGE_BOX_LENGTH as f32,
                        )),
                        ..default()
                    },
                    ..default()
                });
            }
            for i in 0..=NUM_LARGE_BOX_HEIGHT {
                parent.spawn(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        0.,
                        (i as f32 - NUM_LARGE_BOX_HEIGHT as f32 / 2.0) * GRID_LARGE_BOX_LENGTH as f32,
                        1.0,
                    )),
                    sprite: Sprite {
                        color: Color::hsla(1.0, 0.86, 0.42, 0.5),
                        custom_size: Some(Vec2::new(
                            NUM_LARGE_BOX_WIDTH as f32 * GRID_LARGE_BOX_LENGTH as f32,
                            1.5,
                        )),
                        ..default()
                    },
                    ..default()
                });
            }
        });
}

pub fn despawn_grid(
    grid_query: Query<Entity, With<Grid>>,
    mut commands: Commands,
) {
    if let Ok(entity) = grid_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}