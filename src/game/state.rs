use crate::game::ball::Ball;
use crate::game::player::Player;
use crate::game::{ball, player};
use bevy::app::App;
use bevy::prelude::{
    in_state, Commands, Component, IntoSystemConfigs, NextState, Query, States, Transform, With,
};

#[derive(Component)]
pub struct StatePlugin;

impl bevy::app::Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_state(GameState::Running)
            .add_systems(
                bevy::app::FixedUpdate,
                pause_if_collision.run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum GameState {
    #[default]
    Running,
    Pause,
}

fn pause_if_collision(
    mut cmd: Commands,
    player_transform_query: Query<&Transform, With<Player>>,
    ball_transforms_query: Query<&Transform, With<Ball>>,
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
            cmd.insert_resource(NextState(Some(GameState::Pause)))
        }
    }
}
