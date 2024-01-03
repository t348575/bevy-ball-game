use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    events::GameOver,
    game::{
        enemy::{components::Enemy, ENEMY_RADIUS},
        score::resources::Score,
        star::{components::Star, STAR_RADIUS},
    },
};

use super::{components::Player, HALF_PLAYER_SIZE, PLAYER_RADIUS, PLAYER_SPEED};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
                .with_scale(Vec3::splat(0.5)),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..Default::default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let transform = player_query.get_single_mut();
    if transform.is_err() {
        return;
    }

    let mut transform = transform.unwrap();
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if direction.length() > 0.0 {
        direction = direction.normalize()
    }

    transform.translation += direction * PLAYER_SPEED * time.delta_seconds()
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let transform = player_query.get_single_mut();
    if transform.is_err() {
        return;
    }

    let mut player_transform = transform.unwrap();
    let window = window_query.get_single().unwrap();

    let x_min = HALF_PLAYER_SIZE;
    let x_max = window.width() - HALF_PLAYER_SIZE;
    let y_min = HALF_PLAYER_SIZE;
    let y_max = window.height() - HALF_PLAYER_SIZE;

    let mut translation = player_transform.translation;

    if translation.x < x_min {
        translation.x = x_min;
    } else if translation.x > x_max {
        translation.x = x_max;
    }

    if translation.y < y_min {
        translation.y = y_min;
    } else if translation.y > y_max {
        translation.y = y_max;
    }

    player_transform.translation = translation;
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let transform = player_query.get_single_mut();
    if transform.is_err() {
        return;
    }

    let (player_entity, player_transform) = transform.unwrap();
    let mut game_over = false;
    for (_, enemy_transform) in enemy_query.iter() {
        let distance = player_transform
            .translation
            .distance(enemy_transform.translation);

        if distance <= PLAYER_RADIUS + ENEMY_RADIUS {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/explosionCrunch_000.ogg"),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            });

            commands.entity(player_entity).despawn();
            game_over = true;
            game_over_event_writer.send(GameOver {
                score: score.value,
                time: score.time_alive.elapsed(),
            });
            break;
        }
    }

    if game_over {
        for (entity, _) in enemy_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn player_hit_stars(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    let transform = player_query.get_single_mut();
    if transform.is_err() {
        return;
    }

    let (_, player_transform) = transform.unwrap();
    for (star_entity, star_transform) in star_query.iter() {
        let distance = player_transform
            .translation
            .distance(star_transform.translation);

        if distance <= PLAYER_RADIUS + STAR_RADIUS {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/laserLarge_000.ogg"),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            });

            commands.entity(star_entity).despawn();
            score.value += 1;
        }
    }
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.get_single() {
        commands.entity(player).despawn();
    }
}
