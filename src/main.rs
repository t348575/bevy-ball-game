use bevy::{asset::AssetMetaCheck, prelude::*};
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

mod common;
mod events;
mod game;
mod main_menu;
mod styles;
mod systems;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[macro_export]
macro_rules! const_style {
    (Style { $($key:ident : $value:expr),* }) => {
        {
            let mut style = Style::DEFAULT;
            $(style.$key = $value;)*
            style
        }
    };
}
