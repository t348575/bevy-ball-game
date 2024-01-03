use bevy::{app::AppExit, prelude::*};

use crate::{AppState, common::system_with_generic};

use self::{
    components::*,
    systems::layout::*,
};

mod components;
mod styles;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (
                    system_with_generic::<PlayButton, ResMut<NextState<AppState>>>,
                    system_with_generic::<QuitButton, EventWriter<AppExit>>,
                )
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
