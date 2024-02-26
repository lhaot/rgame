use bevy::app::App;
use bevy::prelude::{Camera2dBundle, Commands};

mod player;
mod ball;

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_plugins(player::PlayerPlugin)
        .add_systems(bevy::app::Startup, setup)
        .add_systems(bevy::app::Update, bevy::window::close_on_esc)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
