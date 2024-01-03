use bevy::{app::AppExit, prelude::*};

use crate::{main_menu::components::*, AppState, common::ButtonInteraction};

impl ButtonInteraction<PlayButton> for ResMut<'_, NextState<AppState>> {
    fn interact(&mut self, interaction: Option<Interaction>) {
        if let Some(Interaction::Pressed) = interaction {
            self.set(AppState::Game);
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
