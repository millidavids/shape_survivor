use bevy::prelude::Event;

#[derive(Event)]
pub struct EnemyDeathEvent(pub f32);
