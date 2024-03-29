use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::{components::Star, resources::StarSpawnTimer, NUM_STARS};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_STARS {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y / 2.0, 0.0).with_scale(Vec3::splat(0.5)),
                texture: asset_server.load("sprites/star.png"),
                ..Default::default()
            },
            Star {},
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
    stars: Query<Entity, With<Star>>,
) {
    let num = stars.iter().count();
    if num == NUM_STARS || !star_spawn_timer.timer.finished() {
        return;
    }

    let window = window_query.get_single().unwrap();
    let x = random::<f32>() * window.width();
    let y = random::<f32>() * window.height();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y / 2.0, 0.0).with_scale(Vec3::splat(0.5)),
            texture: asset_server.load("sprites/star.png"),
            ..Default::default()
        },
        Star {},
    ));
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star in star_query.iter() {
        commands.entity(star).despawn();
    }
}
