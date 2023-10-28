use bevy::prelude::{Event, Transform};

#[derive(Event)]
pub struct ExperienceSpawnEvent(pub Transform);

#[derive(Event)]
pub struct SendExperienceEvent(pub f32);