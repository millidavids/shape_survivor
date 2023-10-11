use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct HordeMover {
    pub dxdy: Vec3,
}

impl Default for HordeMover {
    fn default() -> Self {
        HordeMover { dxdy: Vec3::ZERO }
    }
}

impl HordeMover {
    pub fn noise(&mut self) {
        self.dxdy.x = rand::thread_rng().gen_range(-100.0..100.0);
        self.dxdy.y = rand::thread_rng().gen_range(-100.0..100.0);
    }
}