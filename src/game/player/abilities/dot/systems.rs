use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use rand::seq::IteratorRandom;

use crate::game::{
    components::Health,
    enemies::components::Enemy,
    grid::{GRID_HEIGHT, GRID_WIDTH},
    player::{
        abilities::{components::Projectile, DEFAULT_ABILITY_SPEED},
        components::Player,
    },
};

use super::{components::Dot, DEFAULT_DOT_RADIUS};

/// A Bevy system responsible for spawning dot projectiles in the game.
///
/// The `spawn_dot` system spawns dot projectiles, which are typically used as abilities or attacks in a game.
///
/// # Parameters
///
/// - `commands`: A mutable reference to the `Commands` resource, used to issue commands for entity creation and manipulation.
/// - `asset_server`: A resource that manages assets, such as textures.
/// - `texture_atlases`: A mutable reference to the `Assets<TextureAtlas>` resource, used for storing texture atlases.
/// - `player_query`: A query for accessing the `Transform` component of entities with the `Player` component.
/// - `enemy_query`: A query for accessing the `Transform` component of entities with the `Enemy` component.
/// - `dots_query`: A query for accessing entities with the `Dot` component.
///
/// # Description
///
/// The `spawn_dot` system first despawns any existing dot projectiles by iterating through entities with the `Dot` component and despawning them recursively.
///
/// Then, it attempts to retrieve the `Transform` of the player entity and the `Transform` of a random enemy entity using queries. If both queries succeed, it proceeds to spawn a new dot projectile entity with specific properties, including its position, speed, and animation settings. The dot projectile is spawned at the player's location and moves towards the randomly selected enemy.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// impl Plugin for MyDotPlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(Update, spawn_dot); // Add a system to spawn dot projectiles.
///     }
/// }
/// ```
///
/// In the example above, the `spawn_dot` system is added to a Bevy app as a startup system and is responsible for spawning dot projectiles in the game.
///
/// # See Also
///
/// - [`Plugin`](https://bevyengine.github.io/bevy/bevy/app/trait.Plugin.html): Bevy trait for defining plugins.
/// - [`Transform`](https://bevyengine.github.io/bevy/bevy/ecs/struct.Transform.html): Bevy component representing an entity's position and orientation.
/// - [`TextureAtlas`](https://bevyengine.github.io/bevy/bevy/asset/struct.TextureAtlas.html): Bevy asset type for managing texture atlases.
/// - [`Query`](https://bevyengine.github.io/bevy/bevy/ecs/struct.Query.html): Bevy mechanism for accessing entity components.
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

/// A Bevy system responsible for moving dot projectiles in the game.
///
/// The `move_dots` system updates the positions of dot projectiles based on their direction and speed.
///
/// # Parameters
///
/// - `dots_query`: A query for accessing the `Transform` and `Projectile` components of entities with the `Dot` component.
/// - `time`: A resource representing time, used to calculate the movement of dot projectiles.
///
/// # Description
///
/// The `move_dots` system iterates through entities with the `Dot` component and retrieves their `Transform` and `Projectile` components using a query. For each dot projectile entity, it updates the `Transform` to move the dot in its designated direction with a speed factor applied over time.
///
/// Dot projectiles typically represent abilities or attacks in a game and are moved according to their direction and speed.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// impl Plugin for MyDotPlugin {
///     fn build(&self, app: &mut App) {
///         app.add_systems(Update, move_dots); // Add a system to move dot projectiles.
///     }
/// }
/// ```
///
/// In the example above, the `move_dots` system is added to a Bevy app as a regular system and is responsible for updating the positions of dot projectiles in the game.
///
/// # See Also
///
/// - [`Plugin`](https://bevyengine.github.io/bevy/bevy/app/trait.Plugin.html): Bevy trait for defining plugins.
/// - [`Transform`](https://bevyengine.github.io/bevy/bevy/ecs/struct.Transform.html): Bevy component representing an entity's position and orientation.
/// - [`Projectile`](#struct.Projectile): A Bevy component representing the speed and direction of a projectile.
/// - [`Query`](https://bevyengine.github.io/bevy/bevy/ecs/struct.Query.html): Bevy mechanism for accessing entity components.
pub fn move_dots(mut dots_query: Query<(&mut Transform, &Projectile), With<Dot>>, time: Res<Time>) {
    for (mut dot_transform, projectile) in &mut dots_query {
        dot_transform.translation -=
            projectile.direction.normalize_or_zero() * projectile.speed * time.delta_seconds();
    }
}
/// TODO: needs to change to do damage rather than just despawn the entities
pub fn enemy_impact(
    mut commands: Commands,
    mut enemies_query: Query<(&mut Health, &Transform, &Handle<TextureAtlas>), With<Enemy>>,
    dots_query: Query<(Entity, &Transform), With<Dot>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (dot_entity, dot_transform) in &dots_query {
        for (mut enemy_health, enemy_transform, enemy_texture_atlas) in &mut enemies_query {
            if collide(
                dot_transform.translation,
                Vec2::splat(DEFAULT_DOT_RADIUS * 2.0),
                enemy_transform.translation,
                Vec2::splat(texture_atlases.get(enemy_texture_atlas).unwrap().size.y / 2.0),
            ) != None
            {
                commands.entity(dot_entity).despawn_recursive();
                enemy_health.0 -= 100.0;
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
