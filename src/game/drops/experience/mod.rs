mod components;
pub mod events;
mod systems;

use bevy::prelude::*;

use crate::game::states::GameState;

use self::{
    events::{ExperienceSpawnEvent, SendExperienceEvent},
    systems::{player_collect, spawn_experience, pulse_experience},
};

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExperienceSpawnEvent>()
            .add_event::<SendExperienceEvent>()
            .add_systems(
            Update,
            (spawn_experience, pulse_experience, player_collect)
                .run_if(in_state(GameState::Running)),
        );
    }
}
