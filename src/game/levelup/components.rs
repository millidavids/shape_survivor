use bevy::prelude::Component;

#[derive(Component)]
pub struct LevelUpMenu;

#[derive(Component)]
pub enum LevelUpMenuButton {
    Continue,
    Upgrade1,
    Upgrade2,
    Upgrade3,
}
