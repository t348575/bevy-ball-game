use crate::const_style;
use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const GAME_HUD_CONTAINER_STYLE: Style = const_style!(Style {
    display: Display::Flex,
    width: Val::Percent(100.0),
    height: Val::Percent(15.0),
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center
});

pub const SCORE_STYLE: Style = const_style!(Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(200.0),
    height: Val::Px(80.0),
    margin: UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0))
});

pub const TIME_STYLE: Style = const_style!(Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(200.0),
    height: Val::Px(80.0)
});

pub const ENEMY_STYLE: Style = const_style!(Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(200.0),
    height: Val::Px(80.0),
    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(32.0))
});

pub const IMAGE_STYLE: Style = const_style!(Style {
    width: Val::Px(48.0),
    height: Val::Px(48.0),
    margin: UiRect::all(Val::Px(8.0))
});
