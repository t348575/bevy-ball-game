use bevy::prelude::*;

use crate::{
    common::*,
    game::ui::pause::{components::*, styles::*},
};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_CONTAINER_STYLE,
                background_color: BACKGROUND_COLOR.into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Game paused",
                        text_style(asset_server, 48.0),
                    )],
                    ..default()
                },
                ..default()
            });

            build_default_button(parent, asset_server, "Resume", ResumeButton);
            build_default_button(parent, asset_server, "Main menu", MainMenuButton);
            build_default_button(parent, asset_server, "Quit", QuitButton);
        });
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}
