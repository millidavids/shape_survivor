pub mod components;
pub mod events;
mod systems;
mod triangle;

use bevy::prelude::*;

use self::{
    events::EnemyDeathEvent,
    systems::{check_health, damage_enemies},
    triangle::TrianglePlugin,
};

use super::states::GameState;

pub const ENEMY_STD_SPEED: f32 = 125.0;
pub const ENEMY_STD_AVOIDANCE: f32 = 0.2;
pub const ENEMY_STD_SIZE: f32 = 32.0;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDeathEvent>()
            .add_plugins(TrianglePlugin)
            .add_systems(
                Update,
                (damage_enemies, check_health)
                    .chain()
                    .run_if(in_state(GameState::Running)),
            );
    }
}
