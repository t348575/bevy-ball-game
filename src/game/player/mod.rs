use bevy::prelude::*;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

mod components;
mod systems;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const HALF_PLAYER_SIZE: f32 = PLAYER_SIZE / 2.0;
pub const PLAYER_RADIUS: f32 = PLAYER_SIZE / 2.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement),
        )
        .add_systems(OnEnter(AppState::Game), spawn_player)
        .add_systems(OnExit(AppState::Game), despawn_player)
        .add_systems(
            Update,
            (
                player_movement.in_set(PlayerSystemSet::Movement),
                confine_player_movement.in_set(PlayerSystemSet::Confinement),
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        )
        .add_systems(
            Update,
            (enemy_hit_player, player_hit_stars)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
                .chain(),
        );
    }
}
