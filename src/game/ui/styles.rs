use bevy::prelude::*;

pub const UI_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_self = JustifySelf::Start;
    style.align_self = AlignSelf::Start;
    style
};

pub const XP_BAR_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(0.0);
    style.height = Val::Percent(2.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_self = JustifySelf::Start;
    style.align_self = AlignSelf::Start;
    style
};

pub const XP_TEXT_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.margin = UiRect::all(Val::Px(5.0));
    style
};

pub fn get_xp_text(asset_server: &Res<AssetServer>, text: &str) -> Text {
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
                    lightness: 0.1,
                    alpha: 1.0,
                },
            },
        )],
        alignment: TextAlignment::Center,
        ..default()
    }
}