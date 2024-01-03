use bevy::prelude::*;

use crate::game::ui::hud::{components::*, styles::*};

pub fn spawn_game_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_hud(&mut commands, &asset_server);
}

fn build_game_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let default_text = || TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            )],
            ..default()
        },
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: GAME_HUD_CONTAINER_STYLE,
                z_index: ZIndex::Global(100),
                ..default()
            },
            GameHud,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: BACKGROUND_COLOR.into(),
                    style: SCORE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/star.png").into(),
                        ..default()
                    });

                    parent.spawn((default_text(), ScoreText));
                });

            parent
                .spawn(NodeBundle {
                    background_color: BACKGROUND_COLOR.into(),
                    style: TIME_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((default_text(), TimeText));
                });

            parent
                .spawn(NodeBundle {
                    background_color: BACKGROUND_COLOR.into(),
                    style: ENEMY_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });

                    parent.spawn((default_text(), EnemyText));
                });
        });
}

pub fn despawn_game_hud(mut commands: Commands, game_hud_query: Query<Entity, With<GameHud>>) {
    if let Ok(game_hud) = game_hud_query.get_single() {
        commands.entity(game_hud).despawn_recursive();
    }
}
