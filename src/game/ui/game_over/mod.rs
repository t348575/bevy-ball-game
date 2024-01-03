use bevy::{prelude::*, app::AppExit};

use crate::{AppState, common::system_with_generic};

use self::{systems::layout::*, components::*};

mod components;
mod styles;
mod systems;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(
                Update,
                (
                    system_with_generic::<RestartButton, ResMut<NextState<AppState>>>,
                    system_with_generic::<MainMenuButton, ResMut<NextState<AppState>>>,
                    system_with_generic::<QuitButton, EventWriter<AppExit>>,
                )
                    .run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
