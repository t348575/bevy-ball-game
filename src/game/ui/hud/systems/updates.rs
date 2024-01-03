use bevy::prelude::*;

use crate::game::{enemy::components::Enemy, score::resources::Score, ui::hud::components::*};

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        if let Ok(mut score_text) = text_query.get_single_mut() {
            score_text.sections[0].value = score.value.to_string();
        }
    }
}

pub fn update_time_text(mut text_query: Query<&mut Text, With<TimeText>>, score: Res<Score>) {
    if score.is_changed() {
        if let Ok(mut time_text) = text_query.get_single_mut() {
            let elapsed = score.time_alive.elapsed();
            time_text.sections[0].value = format!(
                "{}.{}s",
                elapsed.as_secs(),
                (elapsed.as_millis() - (elapsed.as_secs() as u128 * 1000)) / 10
            );
        }
    }
}

pub fn update_enemies_text(
    mut text_query: Query<&mut Text, With<EnemyText>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let count = enemy_query.iter().count();
    if let Ok(mut enemy_text) = text_query.get_single_mut() {
        enemy_text.sections[0].value = count.to_string();
    }
}
