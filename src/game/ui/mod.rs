use bevy::prelude::*;

mod game_over;
mod hud;
mod pause;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(hud::HudPlugin)
            .add_plugins(pause::PauseMenuPlugin)
            .add_plugins(game_over::GameOverMenuPlugin);
    }
}
