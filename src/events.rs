use std::time::Duration;

use bevy::prelude::*;

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
    pub time: Duration,
}
