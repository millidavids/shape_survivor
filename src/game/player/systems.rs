use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{components::*, PLAYER_SPEED, events::{AddPlayerXpEvent, PlayerLevelUpEvent}};

use crate::game::components::{AnimationIndices, AnimationTimer};

/// Spawns the player entity at the center of the primary window with its associated components.
///
/// The player is represented by a sprite sheet which is segmented into individual frames to be used for animation.
///
/// # System Parameters
/// * `commands`: A mutable reference to the commands for spawning and despawning entities.
/// * `asset_server`: A shared reference to the `AssetServer` which is used to load assets.
/// * `texture_atlases`: A mutable reference to the collection of `TextureAtlas` assets.
/// * `window_query`: A query to retrieve the primary window's dimensions.
///
/// # Behavior
/// 1. Fetches the dimensions of the primary window to position the player at the center.
/// 2. Loads the player's sprite sheet from the specified path ("sprites/circle_player_4_frame_64x64.png").
/// 3. Segments the loaded sprite sheet into a `TextureAtlas`, considering each frame to be 64x64 pixels in size.
/// 4. Defines the range of animation indices for the player using the `AnimationIndices` component.
/// 5. Spawns a new entity with the `Player`, `SpriteSheetBundle`, `AnimationIndices`, and `AnimationTimer` components.
///
/// # Example
/// Assuming you have the necessary assets in the specified paths and the required components and plugins added to your Bevy app:
///
/// ```rust
/// fn main() {
///     App::build()
///         .add_plugins(DefaultPlugins)
///         .add_system(OnEnter(AppState::Game), spawn_player)
///         .run();
/// }
/// ```
///
/// When you run the above app, it initializes Bevy with the default plugins and, during the startup phase, runs the `spawn_player` system, which spawns a player entity at the center of the screen.
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
    ));
}

/// Despawns the player entity from the current Bevy world.
///
/// This system will search for an entity with the `Player` component and, if found, despawn it and its child entities recursively.
///
/// # System Parameters
/// * `player_query`: A query to identify and fetch the player entity.
/// * `commands`: A mutable reference to the commands for spawning and despawning entities.
///
/// # Behavior
/// 1. Uses the `player_query` to find an entity tagged with the `Player` component.
/// 2. If a player entity is found, the system despawns it, ensuring to remove all of its child entities as well.
///
/// # Example
/// Assuming you have previously spawned a player entity using a system like `spawn_player` and now want to remove it from the world:
///
/// ```rust
/// fn main() {
///     App::build()
///         .add_plugins(DefaultPlugins)
///         .add_systems(OnEnter(AppState::Game), spawn_player)  // Assuming `spawn_player` spawns a player
///         .add_systems(OnExit(AppState::Game), despawn_player)
///         .run();
/// }
/// ```
///
/// In the above app setup, after the player entity is spawned during the startup phase, the `despawn_player` system will run in the next frame and remove the player entity.
pub fn despawn_player(player_query: Query<Entity, With<Player>>, mut commands: Commands) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Animates the player's sprite based on elapsed time and specified animation indices.
///
/// This system handles the animation of a player entity's sprite by updating its frame index over time, depending on the defined animation indices and the time elapsed. It leverages the `AnimationIndices` to determine which frames to show and in what order, while the `AnimationTimer` controls the frequency of frame changes.
///
/// # System Parameters:
/// * `time`: The global time resource, which provides delta time between frames.
/// * `query`: A query to identify and fetch the player entity's animation components and current sprite frame.
///
/// # Behavior:
/// 1. The system iterates over entities that have both `Player` and animation components.
/// 2. For each player entity, the system updates its animation timer with the elapsed time.
/// 3. If the timer has completed its cycle (i.e., the duration for the current frame has passed), the system calculates the next frame using `AnimationIndices`.
/// 4. The player's sprite frame index is then updated to display the new frame.
///
/// # Example:
/// Suppose you have a player entity with an animated sprite. By adding the `animate_player` system to your app, the player's sprite will animate over time:
///
/// ```rust
/// fn main() {
///     App::build()
///         .add_plugins(DefaultPlugins)
///         .add_systems(OnEnter(AppState::Game), spawn_player)  // Assuming `spawn_player` spawns a player with animation components
///         .add_systems(Update, animate_player)
///         .run();
/// }
/// ```
///
/// In the above setup, the player entity's sprite will be animated based on the defined animation indices and timer as the game progresses.
pub fn animate_player(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = indices.tick(&sprite.index);
        }
    }
}

/// Moves the player entity based on keyboard input.
///
/// This system is responsible for translating the player entity in the game world based on the player's keyboard input. The arrow keys or the W, A, S, D keys control the player's movement direction.
///
/// # System Parameters:
/// * `keyboard_input`: The global keyboard input resource, providing the state of key presses.
/// * `player_query`: A query to identify and fetch the player entity's transform.
/// * `time`: The global time resource, which provides delta time between frames.
///
/// # Behavior:
/// 1. The system checks for the current pressed keys using the `keyboard_input` resource.
/// 2. Depending on the pressed keys, a direction vector is computed:
///     - W: Increases the Y direction.
///     - S: Decreases the Y direction.
///     - A: Decreases the X direction.
///     - D: Increases the X direction.
/// 3. If there is any movement direction, the system normalizes the direction vector.
/// 4. The player's transform is then updated by translating it in the calculated direction, scaled by `PLAYER_SPEED` and the elapsed time.
///
/// # Example:
/// To have a player entity move based on keyboard inputs in your Bevy app:
///
/// ```rust
/// fn main() {
///     App::build()
///         .add_plugins(DefaultPlugins)
///         .add_systems(OnEnter(AppState::Game), spawn_player)  // Assuming `spawn_player` spawns a player with necessary components
///         .add_systems(Update, move_player)
///         .run();
/// }
/// ```
///
/// With the above setup, the player entity will move in the game world based on the user's keyboard input.
pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if let Ok(mut player_transform) = player_query.get_single_mut() {
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

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

/// Makes the camera follow the player's position.
///
/// This system ensures that the main camera in the game world remains centered on the player entity, creating a follow-camera effect. The camera's translation (x and y) is updated to match the player's current position.
///
/// # System Parameters:
/// * `player_query`: A query to fetch the transform of the player entity.
/// * `camera_query`: A query to fetch the transform of the main camera in the game. This query specifically excludes entities with the `Player` component to ensure it only targets camera entities.
///
/// # Behavior:
/// 1. Fetch the transform of the player entity using `player_query`.
/// 2. Fetch the transform of the camera entity using `camera_query`.
/// 3. Update the camera's x and y translations to match that of the player, ensuring the camera remains centered on the player.
///
/// # Note:
/// It's important that there exists only one player entity and one camera entity for this system to operate correctly. If there are multiple players or cameras, the system will only consider the first instance of each.
///
/// # Example:
/// To use the camera follow functionality in your Bevy app:
///
/// ```rust
/// fn main() {
///     App::build()
///         .add_plugins(DefaultPlugins)
///         .add_systems(StartUp, spawn_camera)  // Assuming `spawn_camera` spawns a camera with the necessary components
///         .add_systems(OnEnter(AppState::Game), spawn_player)  // Assuming `spawn_player` spawns a player with the necessary components
///         .add_systems(Update, camera_follow)
///         .run();
/// }
/// ```
///
/// With the above setup, the camera will continually adjust its position to follow the player's movement in the game world.
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
    mut add_player_xp_event_reader: EventReader<AddPlayerXpEvent>,
    mut player_level_up_event_writer: EventWriter<PlayerLevelUpEvent>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let xp: f32 = add_player_xp_event_reader.iter().map(|e| e.0).sum();
        player.xp.0 += xp;
        if player.xp.0 >= player.xp.1 {
            player_level_up_event_writer.send(PlayerLevelUpEvent);
        }
    }
}

pub fn check_level(
    mut player_level_up_event_reader: EventReader<PlayerLevelUpEvent>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        for _ in player_level_up_event_reader.iter() {
            player.level_up();
            println!("Level Up: {:?}", player);
        }
    }
}