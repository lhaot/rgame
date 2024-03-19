use bevy::app::App;
use bevy::prelude::Component;

pub(crate) mod player;
pub(crate) mod spark;
pub(crate) mod wall;

#[derive(Component)]
pub(crate) struct GamePlugin;

impl bevy::app::Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, spark::SparkPlugin, wall::WallPlugin));
    }
}
