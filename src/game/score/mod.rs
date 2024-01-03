use bevy::prelude::*;

use crate::AppState;

use self::{resources::HighScores, systems::*};

use super::SimulationState;

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_systems(
                OnEnter(AppState::Game),
                insert_score
                    .before(start_timer)
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                OnExit(AppState::Game),
                stop_timer
                    .before(remove_score)
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                update_timer
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(Update, update_high_scores);
    }
}
