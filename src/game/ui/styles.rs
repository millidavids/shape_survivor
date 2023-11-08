use bevy::prelude::*;

pub const XP_BAR_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(0.0);
    style.height = Val::Percent(2.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_self = JustifySelf::Start;
    style.align_self = AlignSelf::Start;
    style
};