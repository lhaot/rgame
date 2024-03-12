use bevy::app::App;
use bevy::prelude::{
    in_state, ButtonInput, Commands, Component, IntoSystemConfigs, KeyCode, NextState, OnEnter,
    OnExit, Res, Update,
};

use crate::state::GameState;

#[derive(Component)]
pub(crate) struct UiPlugin;

impl bevy::app::Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnExit(GameState::Menu), despawn_menu)
            .add_systems(Update, handle_menu_input.run_if(in_state(GameState::Menu)));
    }
}

#[derive(Component)]
struct Menu;

fn spawn_menu() {
    println!("spawn menu");
}

fn handle_menu_input(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        cmd.insert_resource(NextState(Some(GameState::Running)))
    }
}

fn despawn_menu() {
    println!("despawn menu")
}
