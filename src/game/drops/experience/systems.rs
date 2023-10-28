use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::game::{
    components::{AnimationIndices, AnimationTimer},
    drops::components::Drop,
    player::components::Player,
};

use super::{
    components::Experience,
    events::{ExperienceSpawnEvent, SendExperienceEvent},
};

pub fn spawn_experience(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut experience_spawn_event_reader: EventReader<ExperienceSpawnEvent>,

) {
    for event in &mut experience_spawn_event_reader {
        let xp = Experience::generate();
        let texture_handle = asset_server.load(format!("sprites/{}_experience_4_frame_64x64.png", xp.to_string()));
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices {
            first: 0,
            last: 3,
            reverse: false,
        };

        commands.spawn((
            Drop {},
            xp,
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: event.0,
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

pub fn pulse_experience(
    mut query: Query<
        (&TextureAtlasSprite, &mut Transform),
        With<Experience>,
    >,
) {
    for (sprite, mut transform) in &mut query {
        transform.scale = Vec3::splat(sprite.index as f32 * 0.05 + 0.2);
    }
}

pub fn player_collect(
    mut commands: Commands,
    player_query: Query<(&Transform, &Handle<TextureAtlas>), (With<Player>, Without<Experience>)>,
    experience_query: Query<(Entity, &Experience, &Transform), Without<Player>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut send_experience_event_writer: EventWriter<SendExperienceEvent>,
) {
    if let Ok((player_transform, player_texture_atlas)) = player_query.get_single() {
        for (experience_entity, experience, experience_transform) in &experience_query {
            if collide(
                player_transform.translation,
                Vec2::splat(texture_atlases.get(player_texture_atlas).unwrap().size.y / 2.0),
                experience_transform.translation,
                Vec2::splat(1.0),
            ) != None
            {
                commands.entity(experience_entity).despawn_recursive();
                send_experience_event_writer.send(SendExperienceEvent((*experience).into()));
            }
        }
    }
}
