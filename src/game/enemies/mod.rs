pub mod components;
mod events;
mod systems;
mod triangle;

use bevy::prelude::*;

use self::{
    events::EnemyDeathEvent,
    systems::{check_health, enemy_killed},
    triangle::TrianglePlugin,
};

use super::states::GameState;

pub const ENEMY_STD_SPEED: f32 = 125.0;
pub const ENEMY_STD_AVOIDANCE: f32 = 0.2;
pub const ENEMY_STD_SIZE: f32 = 32.0;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    /// Builds the `EnemiesPlugin`, adding related systems and resources to the Bevy app.
    ///
    /// This function is called automatically when adding the `EnemiesPlugin` to your Bevy app using
    /// the `add_plugin` method.
    ///
    /// # Parameters
    /// - `app`: A mutable reference to the Bevy app to which the plugin should add its systems and resources.
    fn build(&self, app: &mut App) {
        // Add systems and resources related to enemy entities and behaviors here
        app.add_event::<EnemyDeathEvent>()
            .add_plugins(TrianglePlugin)
            .add_systems(
                Update,
                ((check_health, enemy_killed).chain()).run_if(in_state(GameState::Running)),
            );
    }
}
