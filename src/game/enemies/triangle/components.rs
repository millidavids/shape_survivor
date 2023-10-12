use bevy::prelude::Component;

/// A Bevy component representing a triangle-shaped enemy entity in the game.
///
/// The `Triangle` component is used to identify and manage triangle enemies within the game world.
/// It can be attached to entities to mark them as triangle enemies and enable specific behaviors or interactions.
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
///         app.add_system(OnEnter(AppState::Game), spawn_triangles);
///         // ... Add other systems and setup as needed.
///     }
/// }
///
/// fn spawn_triangles(mut commands: Commands) {
///     // Spawn triangle enemy entities and attach the `Triangle` component to them.
///     commands.spawn().insert(Triangle {});
///     // ... Spawn more triangle enemies.
/// }
/// ```
///
/// In the example above, the `Triangle` component is inserted into entities to mark them as triangle enemies
/// during the game's setup phase. This allows Bevy systems to interact with or update these entities based on their role.
///
/// # See Also
///
/// - [`Plugin`](https://bevyengine.github.io/bevy/bevy/app/trait.Plugin.html): Bevy trait for defining plugins.
/// - [`spawn_triangles`](fn.spawn_triangles.html): Bevy system for spawning triangle enemies.
/// - [`TrianglePlugin`](struct.TrianglePlugin.html): A Bevy plugin for managing triangle enemy entities.
#[derive(Component)]
pub struct Triangle;