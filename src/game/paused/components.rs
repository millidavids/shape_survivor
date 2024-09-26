use bevy::prelude::Component;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseMenuButton {
    Resume,
    Start,
    MainMenu,
}
