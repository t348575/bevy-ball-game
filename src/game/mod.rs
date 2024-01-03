use bevy::prelude::*;

use crate::{events::GameOver, AppState};

use self::{
    enemy::EnemyPlugin,
    player::PlayerPlugin,
    score::ScorePlugin,
    star::StarPlugin,
    systems::{pause_simulation, resume_simulation, toggle_simulation},
    ui::GameUIPlugin,
};

mod enemy;
mod player;
mod score;
mod star;
mod systems;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_plugins(GameUIPlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    Paused,
    #[default]
    Running,
}
