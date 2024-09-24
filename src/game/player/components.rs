use bevy::prelude::*;

use super::DEFAULT_PLAYER_SPEED;

#[derive(Component, Debug)]
pub struct Player {
    pub xp: (f32, f32),
    pub lv: usize,
    pub health: (f32, f32),
    pub speed: f32,
}

impl Player {
    pub fn level_up(&mut self) {
        self.lv += 1;
        self.xp.1 *= 1.5;
        self.xp.0 = 0.0;
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health.0 -= amount;
        if self.health.0 < 0.0 {
            self.health.0 = 0.0;
            // Handle player death here if needed
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            xp: (0.0, 10.0),
            lv: 0,
            health: (10.0, 10.0),
            speed: DEFAULT_PLAYER_SPEED,
        }
    }
}