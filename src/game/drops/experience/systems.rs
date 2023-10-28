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
    mut experience_spawn_event_reader: EventReader<ExperienceSpawnEvent>,
) {
    for event in &mut experience_spawn_event_reader {
        let xp = Experience::generate();
        let animation_indices = AnimationIndices {
            first: 1,
            last: 4,
            reverse: false,
        };

        commands.spawn((
            Drop {},
            xp,
            Text2dBundle {
                text: Text::from_section(
                    xp.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/Davidfont.otf"),
                        font_size: 64.0,
                        color: Color::BLACK,
                    },
                ),
                transform: event.0,
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

pub fn animate_experience(
    time: Res<Time>,
    mut query: Query<
        (&mut AnimationIndices, &mut AnimationTimer, &mut Transform),
        With<Experience>,
    >,
) {
    for (mut indices, mut timer, mut transform) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.scale =
                Vec3::splat(indices.tick(&((transform.scale.x * 4.0) as usize)) as f32 * 0.25);
        }
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
