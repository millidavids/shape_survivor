pub mod experience;
mod components;
mod systems;

use bevy::prelude::*;

use self::{experience::ExperiencePlugin, systems::move_drops};

use super::states::GameState;

pub const DROPS_DISTANCE_THRESHOLD: f32 = 200.0;

pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExperiencePlugin)
            .add_systems(Update, move_drops.run_if(in_state(GameState::Running)));
    }
}