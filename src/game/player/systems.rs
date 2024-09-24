use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{abilities::dot::components::DotMod, components::*, events::PlayerLevelUpEvent, PLAYER_SPEED};

use crate::game::{
    components::{AnimationIndices, AnimationTimer},
    drops::experience::events::SendExperienceEvent, grid::{GRID_HEIGHT, GRID_WIDTH},
};

use std::time::Duration;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let texture_handle = asset_server.load("sprites/circle_player_4_frame_64x64.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices {
        first: 0,
        last: 3,
        reverse: false,
    };

    commands.spawn((
        Player::default(),
        Name::from("Player"),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0)
                .with_scale(Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 1.0,
                }),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        DotMod {
            interval: Duration::from_millis(1000),
        },
    ));
}

pub fn despawn_player(player_query: Query<Entity, With<Player>>, mut commands: Commands) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Handle<TextureAtlas>), With<Player>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if let Ok((mut player_transform, texture_atlas_handle)) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let new_position = player_transform.translation + direction * PLAYER_SPEED * time.delta_seconds();

        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        let sprite_size = texture_atlas.size * player_transform.scale.truncate();
        let half_width = sprite_size.x / 8.0; // I don't know why this is 8.0, but it works
        let half_height = sprite_size.y / 2.0;

        let clamped_x = new_position.x.clamp(half_width, GRID_WIDTH as f32 - half_width);
        let clamped_y = new_position.y.clamp(half_height, GRID_HEIGHT as f32 - half_height);

        player_transform.translation.x = clamped_x;
        player_transform.translation.y = clamped_y;
    }
}

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

pub fn add_xp(
    mut send_experience_event_reader: EventReader<SendExperienceEvent>,
    mut player_level_up_event_writer: EventWriter<PlayerLevelUpEvent>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let xp: f32 = send_experience_event_reader.iter().map(|e| e.0).sum();
        player.xp.0 += xp;
        if player.xp.0 >= player.xp.1 {
            player.level_up();
            player_level_up_event_writer.send(PlayerLevelUpEvent(player.lv));
        }
    }
}
