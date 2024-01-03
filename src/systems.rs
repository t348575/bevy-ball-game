use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{events::GameOver, game::SimulationState, AppState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for _ in game_over_event_reader.read() {
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if *app_state.get() != AppState::Game {
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if *app_state.get() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_sim_state.set(SimulationState::Paused);
        }
    }
}
