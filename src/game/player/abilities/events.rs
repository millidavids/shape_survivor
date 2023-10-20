use bevy::prelude::{Event, Entity};

#[derive(Event)]
pub struct TransmitDamage {
    pub target: Entity,
    pub damage: f32,
}