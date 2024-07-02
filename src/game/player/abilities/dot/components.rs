use std::time::Duration;

use bevy::prelude::Component;

#[derive(Component)]
pub struct Dot;

#[derive(Component)]
pub struct DotMod {
    pub interval: Duration,
}
