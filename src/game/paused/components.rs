use bevy::prelude::Component;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseMenuButton {
    Running,
    MainMenu,
}
