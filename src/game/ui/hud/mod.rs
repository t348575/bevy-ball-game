use bevy::prelude::*;

use crate::AppState;

use self::systems::{layout::*, updates::*};

mod components;
mod styles;
mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_game_hud)
            .add_systems(
                Update,
                (update_score_text, update_enemies_text, update_time_text)
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_game_hud);
    }
}
