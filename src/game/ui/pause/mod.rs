use bevy::{prelude::*, app::AppExit};

use crate::{game::SimulationState, common::system_with_generic, AppState};

use self::{systems::layout::*, components::*};

mod components;
mod styles;
mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            .add_systems(
                Update,
                (
                    system_with_generic::<ResumeButton, ResMut<NextState<SimulationState>>>,
                    system_with_generic::<MainMenuButton, ResMut<NextState<AppState>>>,
                    system_with_generic::<QuitButton, EventWriter<AppExit>>,
                )
                    .run_if(in_state(SimulationState::Paused)),
            )
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}
