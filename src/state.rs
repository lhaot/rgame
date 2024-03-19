use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Commands, Component, in_state, IntoSystemConfigs, KeyCode, NextState, Plugin
    , Res, States, Update,
};

#[derive(Component)]
pub struct StatePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum GameState {
    #[default]
    Pause,
    Running,
}

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                Update,
                handle_continue_input.run_if(in_state(GameState::Pause)),
            );
    }
}

fn handle_continue_input(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        cmd.insert_resource(NextState(Some(GameState::Running)))
    }
}
