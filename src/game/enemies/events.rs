use bevy::prelude::{Event, Entity};

#[derive(Event)]
pub struct EnemyDeathEvent(pub Entity);