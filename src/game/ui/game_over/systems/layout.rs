use bevy::prelude::*;

use crate::{
    common::*,
    game::{
        score::resources::Score,
        ui::game_over::{components::*, styles::*},
    },
};

pub fn spawn_game_over_menu(
    mut commands: Commands,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            NodeBundle {
                style: GAME_OVER_MENU_CONTAINER_STYLE,
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Game over!",
                        text_style(&asset_server, 48.0),
                    )],
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        &format!("Final score: {}", score.value),
                        text_style(&asset_server, 22.0),
                    )],
                    ..default()
                },
                ..default()
            });

            let elapsed = score.time_alive.elapsed();
            let time_alive = format!(
                "{}.{}s",
                elapsed.as_secs(),
                (elapsed.as_millis() - (elapsed.as_secs() as u128 * 1000)) / 10
            );
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        &format!("Time alive: {}", time_alive),
                        text_style(&asset_server, 22.0),
                    )],
                    ..default()
                },
                ..default()
            });

            build_default_button(parent, &asset_server, "Restart", RestartButton);
            build_default_button(parent, &asset_server, "Main menu", MainMenuButton);
            build_default_button(parent, &asset_server, "Quit", QuitButton);
        });
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}
