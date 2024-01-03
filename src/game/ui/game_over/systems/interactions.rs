use bevy::{app::AppExit, prelude::*};

use crate::{common::ButtonInteraction, game::ui::game_over::components::*, AppState};

impl ButtonInteraction<RestartButton> for ResMut<'_, NextState<AppState>> {
    fn interact(&mut self, interaction: Option<Interaction>) {
        if let Some(Interaction::Pressed) = interaction {
            self.set(AppState::Game);
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