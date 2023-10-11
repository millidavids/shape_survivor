mod triangle;
mod components;

use bevy::prelude::*;

use self::triangle::TrianglePlugin;

pub struct EnemiesPlugin;

pub const ENEMY_STD_SPEED: f32 = 250.0;
pub const ENEMY_STD_AVOIDANCE: f32 = 0.2;
pub const ENEMY_STD_SIZE: f32 = 32.0;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TrianglePlugin);
    }
}
