use bevy::{app::AppExit, prelude::*};

use crate::{
    common::ButtonInteraction,
    game::{ui::pause::components::*, SimulationState},
    AppState,
};

impl ButtonInteraction<ResumeButton> for ResMut<'_, NextState<SimulationState>> {
    fn interact(&mut self, interaction: Option<Interaction>) {
        if let Some(Interaction::Pressed) = interaction {
            self.set(SimulationState::Running);
        }
    }
}

impl ButtonInteraction<MainMenuButton> for ResMut<'_, NextState<AppState>> {
    fn interact(&mut self, interaction: Option<Interaction>) {
        if let Some(Interaction::Pressed) = interaction {
            self.set(AppState::MainMenu);
        }
    }
}

impl ButtonInteraction<QuitButton> for EventWriter<'_, AppExit> {
    fn interact(&mut self, interaction: Option<Interaction>) {
        if let Some(Interaction::Pressed) = interaction {
            self.send(AppExit);
        }
    }
}