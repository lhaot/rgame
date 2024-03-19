use bevy::app::App;
use bevy::prelude::{Camera2dBundle, Commands, Startup, Update};
use bevy::window::close_on_esc;
use bevy::DefaultPlugins;

mod game;
mod state;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((state::StatePlugin, ui::UiPlugin, game::GamePlugin))
        .add_systems(Update, close_on_esc)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
