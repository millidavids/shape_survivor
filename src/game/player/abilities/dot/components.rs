use bevy::prelude::Component;

/// A Bevy component representing a dot projectile or ability.
///
/// The `Dot` component is used to identify entities that represent dot projectiles or abilities in the game.
/// These entities are typically created and managed by the game systems related to dot abilities.
///
/// # See Also
///
/// - [`System`](https://docs.rs/bevy/latest/bevy/ecs/trait.System.html): Bevy trait for defining custom systems.
/// - [Bevy](https://bevyengine.org/): Bevy game engine's official website.
#[derive(Component)]
pub struct Dot;
