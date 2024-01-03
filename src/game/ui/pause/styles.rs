use bevy::prelude::*;

use crate::const_style;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.0, 0.572, 0.792, 0.5);

pub const PAUSE_MENU_CONTAINER_STYLE: Style = const_style!(Style {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    row_gap: Val::Px(8.0)
});
