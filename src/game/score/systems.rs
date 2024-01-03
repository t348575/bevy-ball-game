use bevy::prelude::*;

use crate::events::GameOver;

use super::resources::{HighScores, Score};

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        high_scores
            .scores
            .push(("Player".to_owned(), event.score, event.time));
    }
}

pub fn start_timer(mut score: ResMut<Score>) {
    score.time_alive.reset();
    score.time_alive.unpause();
}

pub fn stop_timer(mut score: ResMut<Score>) {
    score.time_alive.pause();
}

pub fn update_timer(mut score: ResMut<Score>, time: Res<Time>) {
    score.time_alive.tick(time.delta());
}
