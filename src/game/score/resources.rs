use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
    pub time_alive: Stopwatch,
}

#[derive(Resource, Default)]
pub struct HighScores {
    pub scores: Vec<(String, u32, Duration)>,
}
