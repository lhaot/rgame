use bevy::app::App;
use bevy::prelude::{
    in_state, Commands, Component, IntoSystemConfigs, NextState, Query, States, Transform, With,
};

use crate::game::ball::Enemy;
use crate::game::player::Player;
use crate::game::{ball, player};

#[derive(Component)]
pub struct StatePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum GameState {
    #[default]
    Menu,
    Running,
}

impl bevy::app::Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(
            bevy::app::FixedUpdate,
            pause_if_collision.run_if(in_state(GameState::Running)),
        );
    }
}

/// while state is `GameState::Running`.
///
/// check distance between player and enemies.
/// switch to `GameState::Pause` if collied
fn pause_if_collision(
    mut cmd: Commands,
    player_transform_query: Query<&Transform, With<Player>>,
    ball_transforms_query: Query<&Transform, With<Enemy>>,
) {
    let collision_powi2 = (player::PLAYER_RADIUS + ball::BALL_RADIUS).powi(2);
    let (px, py) = (
        player_transform_query.get_single().unwrap().translation.x,
        player_transform_query.get_single().unwrap().translation.y,
    );
    for ball_translation in &ball_transforms_query {
        let (bx, by) = (
            ball_translation.translation.x,
            ball_translation.translation.y,
        );
        let distance_powi2 = (px - bx).powi(2) + (py - by).powi(2);
        if distance_powi2 < collision_powi2 {
            cmd.insert_resource(NextState(Some(GameState::Menu)))
        }
    }
}
