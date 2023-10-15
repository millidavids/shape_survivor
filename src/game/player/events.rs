use bevy::prelude::Event;

#[derive(Event)]
pub struct AddPlayerXpEvent(pub f32);

#[derive(Event)]
pub struct PlayerLevelUpEvent;