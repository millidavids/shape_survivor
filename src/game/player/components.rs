use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub xp: (f32, f32),
    pub lv: usize,
    pub health: (f32, f32),
}

impl Player {
    pub fn level_up(&mut self) {
        self.lv += 1;
        self.xp.1 *= 1.5;
        self.xp.0 = 0.0;
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            xp: (0.0, 10.0),
            lv: 0,
            health: (10.0, 10.0),
        }
    }
}