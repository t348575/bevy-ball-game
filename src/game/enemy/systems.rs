use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::{
    components::Enemy, resources::EnemySpawnTimer, ENEMY_SPEED, HALF_ENEMY_SIZE, NUM_ENEMIES,
};

fn spawn_direction() -> f32 {
    let neg = if random::<bool>() { -1.0 } else { 1.0 };
    neg * random::<f32>()
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_ENEMIES {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(0.5)),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy {
                direction: Vec2::new(spawn_direction(), spawn_direction()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let x_min = HALF_ENEMY_SIZE;
    let x_max = window.width() - HALF_ENEMY_SIZE;
    let y_min = HALF_ENEMY_SIZE;
    let y_max = window.height() - HALF_ENEMY_SIZE;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;

        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            if random::<bool>() {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
            } else {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_002.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
            }
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for mut transform in enemy_query.iter_mut() {
        let x_min = HALF_ENEMY_SIZE;
        let x_max = window.width() - HALF_ENEMY_SIZE;
        let y_min = HALF_ENEMY_SIZE;
        let y_max = window.height() - HALF_ENEMY_SIZE;

        let mut translation = transform.translation;

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

        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if !enemy_spawn_timer.timer.finished() {
        return;
    }

    let window = window_query.get_single().unwrap();
    let x = random::<f32>() * window.width();
    let y = random::<f32>() * window.height();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(0.5)),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..Default::default()
        },
        Enemy {
            direction: Vec2::new(spawn_direction(), spawn_direction()).normalize(),
        },
    ));
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
}
