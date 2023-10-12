use bevy::prelude::*;

/// `Player` Component: A marker component to identify player entities within the game.
///
/// This component doesn't hold any data but is used to tag entities as players. This way, systems can specifically target or exclude player entities by querying for this component.
///
/// # Usage:
/// Assigning the `Player` component to an entity marks it as a player:
/// ```rust
/// commands.spawn().insert(Player);
/// ```
/// You can then use Bevy's `Query` to find or operate on all entities that are players:
/// ```rust
/// fn my_system(query: Query<&Transform, With<Player>>) {
///     // This will loop through all player entities.
///     for transform in query.iter() {
///         // Do something with each player's transform...
///     }
/// }
/// ```
#[derive(Component)]
pub struct Player;