use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::{
    components::{AnimationIndices, AnimationTimer, Health},
    enemies::{
        components::{Enemy, HordeMover},
        ENEMY_STD_SPEED, ENEMY_STD_AVOIDANCE, ENEMY_STD_SIZE,
    },
    player::components::Player,
};

use super::components::Triangle;

/// Spawns triangle enemy entities around the player.
///
/// The `spawn_triangles` system is responsible for creating triangle enemy entities and positioning them around the player.
/// It ensures that there are no more than 5 enemy entities in the scene at any given time to avoid overpopulating the game world.
///
/// The system retrieves the player's position and the window dimensions to calculate the spawn locations for the triangle enemies.
/// It randomizes the angles at which the enemies are spawned around the player, creating a dynamic and unpredictable enemy pattern.
///
/// # Parameters
///
/// - `commands`: A mutable reference to the Bevy `Commands` resource for spawning entities.
/// - `asset_server`: A resource for loading game assets like textures.
/// - `texture_atlases`: A mutable reference to the `TextureAtlas` assets to create texture atlases.
/// - `player_query`: A Bevy query for retrieving the player's `Transform` component.
/// - `window_query`: A Bevy query for retrieving the primary window's dimensions.
/// - `enemy_query`: A Bevy query for counting existing enemy entities.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// struct TrianglePlugin;
///
/// impl Plugin for TrianglePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_systems(OnEnter(AppState::Game), spawn_triangles);
///     }
/// }
/// ```
///
/// In the example above, the `spawn_triangles` system can be added to a Bevy app as part of a game plugin (`TrianglePlugin`).
/// It spawns triangle enemy entities around the player as the game starts.
///
/// # Notes
///
/// - This system ensures that there are no less than 5 enemy entities in the scene at any given time.
/// - The triangle enemies are randomly positioned around the player.
/// - The spawn locations are determined based on the player's position and window dimensions.
/// - The system uses an animation atlas to animate the triangle enemy entities.
///
/// # See Also
///
/// - [`Player`](struct.Player.html): The player component used to retrieve the player's position.
/// - [`Enemy`](struct.Enemy.html): The enemy component indicating that an entity is an enemy.
/// - [`HordeMover`](struct.HordeMover.html): The component used to apply movement to horde entities.
/// - [`TextureAtlas`](https://bevyengine.org/docs/bevy/assets/struct.TextureAtlas.html): Bevy's texture atlas for handling sprite animations.
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

/// Despawns triangle enemy entities.
///
/// The `despawn_triangles` system is responsible for despawning triangle enemy entities from the game world.
/// It iterates through all entities with the `Triangle` component (representing triangle enemies) and despawns them recursively.
///
/// # Parameters
///
/// - `player_query`: A Bevy query for retrieving entities with the `Triangle` component.
/// - `commands`: A mutable reference to the Bevy `Commands` resource for despawning entities.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// struct TrianglePlugin;
///
/// impl Plugin for TrianglePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_systems(StartUp, setup_game);
///         app.add_systems(OnExit(AppState::Game), despawn_triangles);
///     }
/// }
///
/// fn setup_game(mut commands: Commands) {
///     commands.spawn().insert(Triangle {});
///     // ... Spawn more game entities, including triangle enemies.
/// }
/// ```
///
/// In the example above, the `despawn_triangles` system is added to a Bevy app as part of a game plugin (`TrianglePlugin`).
/// It despawns any entities with the `Triangle` component in the game world.
///
/// # Notes
///
/// - This system is typically used to clean up or remove specific types of entities, such as triangle enemies.
/// - It iterates through all entities with the `Triangle` component and recursively despawns them.
///
/// # See Also
///
/// - [`Triangle`](struct.Triangle.html): The component used to identify triangle enemy entities.
/// - [`Commands`](https://bevyengine.org/docs/bevy/app/struct.Commands.html): Bevy's command system for entity manipulation.
pub fn despawn_triangles(player_query: Query<Entity, With<Triangle>>, mut commands: Commands) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Animates triangle enemy entities.
///
/// The `animate_triangle` system handles the animation of triangle enemy entities in the game.
/// It iterates through entities with the `Triangle` component (representing triangle enemies) and updates their animations.
///
/// # Parameters
///
/// - `time`: A Bevy resource representing time, used to control animation timing.
/// - `query`: A Bevy query for selecting entities with the `Triangle` component and related animation data.
///
/// # Animation Logic
///
/// This system advances the animation frame for each triangle enemy based on the `AnimationIndices`, `AnimationTimer`,
/// and `TextureAtlasSprite` components associated with them. It ticks the timer and updates the sprite index when the timer finishes.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// struct TrianglePlugin;
///
/// impl Plugin for TrianglePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_systems(Update, animate_triangle);
///     }
/// }
/// ```
///
/// In the example above, the `animate_triangle` system is added to a Bevy app as part of a game plugin (`TrianglePlugin`).
/// It animates triangle enemy entities based on their animation components.
///
/// # See Also
///
/// - [`Triangle`](struct.Triangle.html): The component used to identify triangle enemy entities.
/// - [`AnimationIndices`](struct.AnimationIndices.html): Component for managing animation frame indices.
/// - [`AnimationTimer`](struct.AnimationTimer.html): Component for controlling animation timing.
/// - [`TextureAtlasSprite`](https://bevyengine.org/docs/bevy/asset/struct.TextureAtlasSprite.html): Bevy component for sprite rendering.
/// - [`Time`](https://bevyengine.org/docs/bevy/app/struct.Time.html): Bevy resource representing time.
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

/// Updates the `HordeMover` components to track the direction towards the player.
///
/// The `direction_to_player` system calculates and updates the `HordeMover` components for triangle enemy entities,
/// causing them to track the direction towards the player.
///
/// # Parameters
///
/// - `triangle_query`: A Bevy query for selecting entities with the `Triangle` component and `HordeMover` components.
/// - `player_query`: A Bevy query for selecting the player entity's `Transform` component.
///
/// # Description
///
/// This system calculates the direction vector from each triangle enemy to the player's position
/// and adds it to their existing `HordeMover` component's `dxdy` field. This causes the triangle enemies to move
/// towards the player over time.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// struct TrianglePlugin;
///
/// impl Plugin for TrianglePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(Update, direction_to_player);
///     }
/// }
///
/// fn setup_game(mut commands: Commands) {
///     commands.spawn().insert(Triangle {}).insert(HordeMover::default());
///     // ... Spawn more game entities, including triangle enemies.
/// }
/// ```
///
/// In the example above, the `direction_to_player` system is added to a Bevy app as part of a game plugin (`TrianglePlugin`).
/// It calculates the direction towards the player for each triangle enemy entity with a `HordeMover` component.
///
/// # See Also
///
/// - [`Triangle`](struct.Triangle.html): The component used to identify triangle enemy entities.
/// - [`HordeMover`](struct.HordeMover.html): Component for tracking movement direction.
/// - [`Transform`](https://bevyengine.org/docs/bevy/ecs/struct.Transform.html): Bevy component for entity transformation.
/// - [`Player`](struct.Player.html): The component used to identify the player entity.
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

/// Updates the `HordeMover` components to avoid collisions with other triangle enemies.
///
/// The `avoid_other_triangles` system calculates and updates the `HordeMover` components for triangle enemy entities
/// to avoid collisions with other triangle enemies.
///
/// # Parameters
///
/// - `triangle_query`: A Bevy query for selecting entities with the `Triangle` component and `HordeMover` components.
///
/// # Description
///
/// This system iterates through pairs of triangle enemy entities and checks if they are within a certain distance
/// of each other. If they are, it calculates a repulsive force between them and updates their `HordeMover` components
/// accordingly. This results in the triangle enemies avoiding collisions and spacing themselves out.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// struct TrianglePlugin;
///
/// impl Plugin for TrianglePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_systems(avoid_other_triangles);
///     }
/// }
/// ```
///
/// In the example above, the `avoid_other_triangles` system is added to a Bevy app as part of a game plugin (`TrianglePlugin`).
/// It calculates repulsive forces between triangle enemy entities with `HordeMover` components, ensuring they avoid collisions.
///
/// # See Also
///
/// - [`Triangle`](struct.Triangle.html): The component used to identify triangle enemy entities.
/// - [`HordeMover`](struct.HordeMover.html): Component for tracking movement direction.
/// - [`Transform`](https://bevyengine.org/docs/bevy/ecs/struct.Transform.html): Bevy component for entity transformation.
pub fn avoid_other_triangles(
    mut triangle_query: Query<(&Transform, &mut HordeMover), (With<Triangle>, Without<Player>)>,
) {
    let mut combinations = triangle_query.iter_combinations_mut();
    while let Some([(transform_a, mut hordemover_a), (transform_b, mut hordemover_b)]) =
        combinations.fetch_next()
    {
        let weight = (ENEMY_STD_SIZE * 1.5) - transform_a.translation.distance(transform_b.translation);
        if weight > 0.0 {
            hordemover_a.dxdy -= (transform_a.translation - transform_b.translation) * weight * ENEMY_STD_AVOIDANCE;
            hordemover_b.dxdy -= (transform_b.translation - transform_a.translation) * weight * ENEMY_STD_AVOIDANCE;
        }
    }
}

/// Moves triangle enemy entities based on their `HordeMover` components.
///
/// The `move_triangle` system updates the position of triangle enemy entities by applying movement vectors
/// stored in their `HordeMover` components.
///
/// # Parameters
///
/// - `triangle_query`: A Bevy query for selecting entities with the `Triangle` component and `HordeMover` components.
/// - `time`: A Bevy resource providing time-related information.
///
/// # Description
///
/// This system iterates through triangle enemy entities that have both `Triangle` and `HordeMover` components.
/// It calculates the new position of each entity by subtracting the movement vector stored in their `HordeMover`
/// component from their current position, scaled by the elapsed time and a constant speed factor (`ENEMY_STD_SPEED`).
///
/// Additionally, a random noise vector is added to the `HordeMover` component of each entity using the `noise` function,
/// creating more unpredictable movement patterns for the triangle enemies.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// struct TrianglePlugin;
///
/// impl Plugin for TrianglePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_systems(Update, move_triangle);
///     }
/// }
/// ```
///
/// In the example above, the `move_triangle` system is added to a Bevy app as part of a game plugin (`TrianglePlugin`).
/// It moves triangle enemy entities based on their `HordeMover` components and adds random noise to their movements.
///
/// # See Also
///
/// - [`Triangle`](struct.Triangle.html): The component used to identify triangle enemy entities.
/// - [`HordeMover`](struct.HordeMover.html): Component for tracking movement direction and noise generation.
/// - [`Transform`](https://bevyengine.org/docs/bevy/ecs/struct.Transform.html): Bevy component for entity transformation.
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
