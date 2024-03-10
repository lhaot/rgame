use bevy::app::App;
use bevy::prelude::Component;

pub(crate) mod ball;
pub(crate) mod player;
pub(crate) mod wall;

#[derive(Component)]
pub struct GamePlugin;

impl bevy::app::Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, ball::EnemyPlugin, wall::WallPlugin));
    }
}
