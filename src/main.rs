use bevy::app::App;
use bevy::prelude::{Camera2dBundle, Commands};

mod ball;
mod player;
mod wall;

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_systems(bevy::app::Startup, setup)
        .add_plugins((player::PlayerPlugin, ball::EnemyPlugin, wall::WallPlugin))
        .add_systems(bevy::app::Update, bevy::window::close_on_esc)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
