use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::hsla(0.0, 0.0, 0.15, 1.0);
pub const HOVERED_BUTTON_COLOR: Color = Color::hsla(0.0, 0.0, 0.25, 1.0);
pub const PRESSED_BUTTON_COLOR: Color = Color::hsla(125.0, 0.75, 0.5, 1.0);

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const NORMAL_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};

pub fn get_button_text(asset_server: &Res<AssetServer>, text: &str) -> Text {
    get_text(asset_server, text, 48.0)
}

fn get_text(asset_server: &Res<AssetServer>, text: &str, font_size: f32) -> Text {
    Text {
        sections: vec![TextSection::new(
            text,
            TextStyle {
                font: asset_server.load("fonts/Davidfont.otf"),
                font_size: font_size,
                color: Color::Hsla {
                    hue: 0.0,
                    saturation: 0.0,
                    lightness: 0.9,
                    alpha: 1.0,
                },
            },
        )],
        alignment: TextAlignment::Center,
        ..default()
    }
}