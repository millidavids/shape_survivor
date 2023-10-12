use bevy::prelude::*;


use super::{
    components::Grid, GRID_LARGE_BOX_LENGTH, GRID_SMALL_BOX_LENGTH,
    NUM_LARGE_BOX_HEIGHT, NUM_LARGE_BOX_WIDTH, NUM_SMALL_BOX_HEIGHT, NUM_SMALL_BOX_WIDTH, GRID_WIDTH, GRID_HEIGHT,
};

/// Spawns a visual grid in the game world.
///
/// The system generates a primary grid, accompanied by both small and large boxes to
/// visualize the subdivisions of the grid. These subdivisions can be useful for alignment,
/// positioning, and various gameplay mechanics.
///
/// # Parameters:
/// - `commands`: A mutable reference to the `Commands` struct, which allows for spawning entities and components.
///
/// # Constants:
/// - `GRID_WIDTH` and `GRID_HEIGHT`: Define the dimensions of the primary grid.
/// - `GRID_SMALL_BOX_LENGTH`: Defines the side length of a small box within the grid.
/// - `NUM_SMALL_BOX_WIDTH` and `NUM_SMALL_BOX_HEIGHT`: Define the number of small boxes in width and height respectively.
/// - `GRID_LARGE_BOX_LENGTH`: Defines the side length of a large box within the grid.
/// - `NUM_LARGE_BOX_WIDTH` and `NUM_LARGE_BOX_HEIGHT`: Define the number of large boxes in width and height respectively.
///
/// # Examples:
/// ```rust
/// # use bevy::prelude::*;
/// # const GRID_WIDTH: f32 = 100.0;
/// # const GRID_HEIGHT: f32 = 100.0;
/// # const GRID_SMALL_BOX_LENGTH: f32 = 10.0;
/// # const NUM_SMALL_BOX_WIDTH: usize = 10;
/// # const NUM_SMALL_BOX_HEIGHT: usize = 10;
/// # const GRID_LARGE_BOX_LENGTH: f32 = 20.0;
/// # const NUM_LARGE_BOX_WIDTH: usize = 5;
/// # const NUM_LARGE_BOX_HEIGHT: usize = 5;
/// #
/// # fn main() {
/// let mut app = App::build();
/// // Add the spawn_grid system to your app's update stage.
/// app.add_systems(OnEnter(AppState::Game), spawn_grid);
/// # }
/// ```
///
/// # Note:
/// Ensure the constants mentioned above are defined in the same module or are globally accessible for this system to function correctly.

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

/// Despawns the visual grid and its associated entities from the game world.
///
/// The system identifies the primary grid by querying entities with the `Grid` component. 
/// Once the grid entity is found, it and all of its child entities are despawned recursively.
///
/// # Parameters:
/// - `grid_query`: A `Query` that identifies entities with the `Grid` component.
/// - `commands`: A mutable reference to the `Commands` struct, which facilitates the despawning of entities and components.
///
/// # Usage:
/// This system can be used to clear the visual grid, especially in cases where the grid needs to 
/// be refreshed, updated, or when transitioning between different game states or levels.
///
/// # Examples:
/// ```rust
/// # use bevy::prelude::*;
///
/// # fn main() {
/// let mut app = App::build();
/// // Add the despawn_grid system to your app's update stage.
/// app.add_systems(OnExit(AppState::Game), despawn_grid);
/// # }
/// ```
///
/// # Note:
/// Ensure that the `Grid` component is defined in the same module or is globally accessible for this system to function correctly.
pub fn despawn_grid(
    grid_query: Query<Entity, With<Grid>>,
    mut commands: Commands,
) {
    if let Ok(entity) = grid_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}