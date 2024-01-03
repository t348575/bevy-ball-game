use crate::const_style;
use bevy::prelude::*;

pub const TITLE_STYLE: Style = const_style!(Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center
});

pub const MAIN_MENU_CONTAINER_STYLE: Style = const_style!(Style {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    row_gap: Val::Px(8.0)
});

pub const IMAGE_STYLE: Style = const_style!(Style {
    width: Val::Px(64.0),
    height: Val::Px(64.0),
    margin: UiRect::all(Val::Px(8.0))
});
