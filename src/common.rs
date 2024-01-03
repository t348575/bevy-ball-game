use bevy::{
    ecs::system::{SystemParam, SystemParamItem},
    prelude::*,
};

use crate::styles::*;

pub fn interact_with_button<T: Component>(
    button_query: &mut Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<T>)>,
) -> Option<Interaction> {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => *background_color = PRESSED_BUTTON_COLOR.into(),
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
        return Some(*interaction);
    }
    None
}

pub fn text_style(asset_server: &Res<AssetServer>, font_size: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
        font_size,
        color: Color::WHITE,
    }
}

pub fn default_text(asset_server: &Res<AssetServer>, text: &str, font_size: f32) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection::new(text, text_style(asset_server, font_size))],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

pub fn build_default_button<T: Component>(
    parent: &mut ChildBuilder<'_, '_, '_>,
    asset_server: &Res<AssetServer>,
    text: &str,
    component: T,
) {
    parent
        .spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            component,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(text, text_style(asset_server, 22.0))],
                    ..default()
                },
                ..default()
            });
        });
}

pub fn node_bundle(style: Style) -> NodeBundle {
    NodeBundle { style, ..default() }
}

pub trait ButtonInteraction<C: Component> {
    fn interact(&mut self, interaction: Option<Interaction>);
}

pub fn system_with_generic<C: Component, T: SystemParam>(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<C>)>,
    mut param: SystemParamItem<T>,
) where
    for<'w, 's> T::Item<'w, 's>: ButtonInteraction<C>,
{
    param.interact(interact_with_button(&mut button_query));
}
