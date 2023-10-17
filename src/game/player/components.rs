use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub xp: (f32, f32),
    pub lv: usize,
}

impl Player {
    pub fn level_up(&mut self) {
        self.lv += 1;
        self.xp.1 *= 1.5;
    }
}

impl Default for Player {
    fn default() -> Self {
        Player { xp: (0.0, 2.0), lv: 0 }
    }
}