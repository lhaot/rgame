mod game;

use bevy::app::App;
use bevy::prelude::{Camera2dBundle, Commands};

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_systems(bevy::app::Startup, setup)
        .add_plugins((
            game::player::PlayerPlugin,
            game::ball::EnemyPlugin,
            game::wall::WallPlugin,
            game::state::StatePlugin,
        ))
        .add_systems(bevy::app::Update, bevy::window::close_on_esc)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
