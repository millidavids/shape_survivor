use bevy::prelude::{Component, Vec3};

use super::DEFAULT_ABILITY_SPEED;

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
