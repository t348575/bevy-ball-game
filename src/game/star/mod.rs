use bevy::prelude::*;

use crate::AppState;

use self::{resources::StarSpawnTimer, systems::*};

use super::SimulationState;

pub mod components;
mod resources;
mod systems;

pub const NUM_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_RADIUS: f32 = STAR_SIZE / 2.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(OnExit(AppState::Game), despawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
                    .chain(),
            );
    }
}
