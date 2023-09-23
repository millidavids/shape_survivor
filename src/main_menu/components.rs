use bevy::prelude::Component;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum MainMenuButton {
    Play,
    Quit,
}
