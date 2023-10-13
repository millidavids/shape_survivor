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

/// Retrieves a `Text` component for a button with the specified content and a default font size.
///
/// This function utilizes the provided `AssetServer` to generate a `Text` component with the given content.
/// It uses a default font size of 48.0 for the text.
///
/// # Parameters
///
/// - `asset_server`: A reference to Bevy's asset server. This is used to load assets, such as fonts, required for the text rendering.
/// - `text`: The string content for the `Text` component. Represents the label or content of the button.
///
/// # Returns
///
/// Returns a `Text` component ready to be used in Bevy's UI systems.
///
/// # Usage
///
/// ```rust
/// use your_crate_name::your_module_name::get_button_text;
///
/// fn some_system(asset_server: Res<AssetServer>) {
///     let play_button_text = get_button_text(&asset_server, "Play");
///     // ... use play_button_text in UI creation or updates ...
/// }
/// ```
///
/// Note: Ensure that the `get_text` function, which this function calls internally, is defined and accessible within the same module or crate.
///
pub fn get_button_text(asset_server: &Res<AssetServer>, text: &str) -> Text {
    get_text(asset_server, text, 48.0)
}

/// Retrieves a `Text` component styled with the specified content, font, and font size.
///
/// This utility function utilizes the provided `AssetServer` to generate a `Text` component styled with a specific font and font size.
/// The default font used is "Davidfont.otf" from the assets directory. The text's color is set to a very light shade (90% lightness),
/// and its alignment is centered.
///
/// # Parameters
///
/// - `asset_server`: A reference to Bevy's asset server. This is used to load the font asset required for the text rendering.
/// - `text`: The string content for the `Text` component. Represents the text's actual content.
/// - `font_size`: The desired font size for the text rendering.
///
/// # Returns
///
/// Returns a styled `Text` component ready to be used in Bevy's UI systems.
///
/// # Usage
///
/// ```rust
/// // Assuming the function is in the same module:
/// fn some_system(asset_server: Res<AssetServer>) {
///     let header_text = get_text(&asset_server, "Welcome!", 72.0);
///     // ... use header_text in UI creation or updates ...
/// }
/// ```
///
/// Note: This function is internally used by the `get_button_text` function to set the default font size for button text.
///
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
