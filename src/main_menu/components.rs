use bevy::prelude::Component;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum MenuButton {
    Play,
    Quit,
}
