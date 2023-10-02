mod triangle;
mod components;

use bevy::prelude::*;

use self::triangle::TrianglePlugin;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TrianglePlugin);
    }
}
