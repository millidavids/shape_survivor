use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::hsla(0.0, 0.0, 0.15, 1.0);
pub const HOVERED_BUTTON_COLOR: Color = Color::hsla(0.0, 0.0, 0.25, 1.0);
pub const PRESSED_BUTTON_COLOR: Color = Color::hsla(125.0, 0.75, 0.5, 1.0);

pub const PAUSE_MENU_STYLE: Style = {
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

pub const PAUSE_MENU_TRANSFORM: Transform = {
    let mut transform = Transform::IDENTITY;
    transform.translation.z = 1000.0;
    transform
};

pub const PAUSE_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};

/// Retrieves a `Text` component with the specified string and a default font size for buttons.
///
/// This function serves as a helper to quickly retrieve a `Text` component with a specified string
/// for use within UI buttons. It utilizes a default font size specifically tailored for buttons.
///
/// # Parameters:
/// - `asset_server`: A reference to the `AssetServer` resource, used to load the desired font asset.
/// - `text`: The string content to be displayed within the `Text` component.
///
/// # Returns:
/// A `Text` component initialized with the given string, default button font size, and other pre-defined styles.
///
/// # Examples:
/// ```rust
/// let text = get_button_text(&asset_server, "Play");
/// commands.spawn(TextBundle { text, ..Default::default() });
/// ```
pub fn get_button_text(asset_server: &Res<AssetServer>, text: &str) -> Text {
    get_text(asset_server, text, 48.0)
}

/// Retrieves a `Text` component with the specified string and font size.
///
/// This utility function is designed to provide a flexible means of obtaining a `Text` component
/// with custom styles, while ensuring consistent design elements like font and color.
///
/// # Parameters:
/// - `asset_server`: A reference to the `AssetServer` resource, used to load the desired font asset.
/// - `text`: The string content to be displayed within the `Text` component.
/// - `font_size`: The desired size of the font.
///
/// # Returns:
/// A `Text` component initialized with the given string, font size, and other pre-defined styles.
///
/// # Examples:
/// ```rust
/// let header_text = get_text(&asset_server, "Game Over", 72.0);
/// commands.spawn(TextBundle { text: header_text, ..Default::default() });
/// ```
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
