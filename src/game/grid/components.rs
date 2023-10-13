use bevy::prelude::Component;

/// Represents a visual grid entity in the game world.
///
/// The `Grid` component is used to tag entities as a part of the visual grid. By associating this component with an entity,
/// systems and queries can efficiently identify and operate on the grid and its associated elements.
///
/// Typically, this component is used alongside visual components, such as `SpriteBundle`, to create a grid in the game scene.
/// The actual layout and appearance of the grid might be defined by associated systems that spawn or manipulate grid entities.
///
/// # Usage:
/// Attach the `Grid` component to an entity to mark it as a part of the visual grid.
///
/// # Examples:
/// ```rust
/// # use bevy::prelude::*;
///
/// fn spawn_grid_system(mut commands: Commands) {
///     commands.spawn(SpriteBundle {
///         // ... your sprite configurations
///     })
///     .with(Grid);
/// }
/// ```
///
/// # Note:
/// The `Grid` component itself doesn't contain any data; it acts as a marker or a tag. Systems in your game logic
/// should use this component to select or filter grid-related entities when necessary.
#[derive(Component)]
pub struct Grid;
