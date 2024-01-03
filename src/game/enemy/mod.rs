use bevy::prelude::*;

use crate::AppState;

use self::{resources::EnemySpawnTimer, systems::*};

use super::SimulationState;

pub mod components;
mod resources;
mod systems;

pub const NUM_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const HALF_ENEMY_SIZE: f32 = ENEMY_SIZE / 2.0;
pub const ENEMY_RADIUS: f32 = ENEMY_SIZE / 2.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(OnExit(AppState::Game), despawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
                    .chain(),
            );
    }
}
