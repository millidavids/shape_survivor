use bevy::prelude::*;

pub const UI_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::ColumnReverse;
    style.justify_self = JustifySelf::Start;
    style.align_self = AlignSelf::Start;
    style.padding = UiRect::all(Val::Percent(1.0));
    style
};

pub const FILLING_BAR_CONTAINER_STYLE_BOTTOM: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Auto;
    style.flex_direction = FlexDirection::Row;
    style.justify_self = JustifySelf::Start;
    style.align_self = AlignSelf::Center;
    style.align_items = AlignItems::Center;
    style.bottom = Val::Px(0.0);
    style.border = UiRect::all(Val::Px(1.0));
    style.padding = UiRect::all(Val::Px(2.0));
    style
};

pub const FILLING_BAR_CONTAINER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Auto;
    style.flex_direction = FlexDirection::Row;
    style.justify_self = JustifySelf::Start;
    style.align_self = AlignSelf::Center;
    style.align_items = AlignItems::Center;
    style.bottom = Val::Px(0.0);
    style.border = UiRect {
        left: Val::Px(1.0),
        right: Val::Px(1.0),
        top: Val::Px(1.0),
        bottom: Val::Px(0.0),
    };
    style.padding = UiRect::all(Val::Px(2.0));
    style
};

pub const FILLING_BAR_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(0.0);
    style.height = Val::Percent(50.0);
    style.flex_direction = FlexDirection::Row;
    style
};

pub const FILLING_BAR_TEXT_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.margin = UiRect::all(Val::Px(5.0));
    style.flex_shrink = 0.0;
    style
};

pub fn get_filling_bar_text(asset_server: &Res<AssetServer>, text: &str) -> Text {
    Text {
        sections: vec![TextSection::new(
            text,
            TextStyle {
                font: asset_server.load("fonts/Davidfont.otf"),
                font_size: 24.0,
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