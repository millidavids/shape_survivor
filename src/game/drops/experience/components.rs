use std::fmt::Display;

use bevy::prelude::Component;
use rand::{Rng, thread_rng};

#[derive(Component, Copy, Clone)]
pub enum Experience {
     X,
    Y,
    Z,
}

impl Experience {
    pub fn generate() -> Self {
        let mut rng = thread_rng();

        match rng.gen_range(1..=10000) {
            x if x % 10000 == 0 => Experience::Z,
            y if y % 100 == 0 => Experience::Y,
            _ => Experience::X,
        }
    }
}

impl Display for Experience {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Experience::X => write!(f, "X"),
            Experience::Y => write!(f, "Y"),
            Experience::Z => write!(f, "Z"),
        }
    }
}

impl Into<f32> for Experience {
    fn into(self) -> f32 {
        match self {
            Experience::X => 1.0,
            Experience::Y => 100.0,
            Experience::Z => 10000.0,
        }
    }
}