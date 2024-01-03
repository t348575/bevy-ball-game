use bevy::prelude::*;

use crate::{
    common::*,
    main_menu::{
        components::{MainMenu, PlayButton, QuitButton},
        styles::*,
    },
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_load = |sprite_name: &str| ImageBundle {
        style: IMAGE_STYLE,
        image: asset_server.load(format!("sprites/{sprite_name}")).into(),
        ..default()
    };

    commands
        .spawn((node_bundle(MAIN_MENU_CONTAINER_STYLE), MainMenu))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(image_load("ball_blue_large.png"));
                    parent.spawn(default_text(&asset_server, "Bevy ball game", 64.0));
                    parent.spawn(image_load("ball_red_large.png"));
                });

            build_default_button(parent, &asset_server, "Play", PlayButton);
            build_default_button(parent, &asset_server, "Quit", QuitButton);
        });
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}
