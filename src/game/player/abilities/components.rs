use bevy::prelude::{Component, Vec3};

use super::DEFAULT_ABILITY_SPEED;

/// A Bevy component representing a projectile fired by a character.
///
/// The `Projectile` component is used to define and store properties of projectiles, such as their speed and direction.
///
/// # Fields
///
/// - `speed`: A `f32` value representing the speed at which the projectile moves.
/// - `direction`: A `Vec3` (3D vector) representing the direction in which the projectile moves.
///
/// # Default Implementation
///
/// The `Default` trait is implemented for `Projectile`, providing a default instance with zero speed and a zero-direction vector.
///
/// # See Also
///
/// - [`spawn_projectile`](fn.spawn_projectile.html): A Bevy system that demonstrates how to spawn projectiles.
/// - [`Vec3`
#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub direction: Vec3,
}

impl Default for Projectile {
    fn default() -> Self {
        Projectile {
            speed: DEFAULT_ABILITY_SPEED,
            direction: Vec3::ZERO,
        }
    }
}
