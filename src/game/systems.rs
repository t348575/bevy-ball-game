use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simulation(mut simulation_next_state: ResMut<NextState<SimulationState>>) {
    simulation_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_next_state: ResMut<NextState<SimulationState>>) {
    simulation_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Paused => {
                commands.insert_resource(NextState(Some(SimulationState::Running)))
            }
            SimulationState::Running => {
                commands.insert_resource(NextState(Some(SimulationState::Paused)))
            }
        }
    }
}
