use bevy::app::App;
use bevy::asset::Assets;
use bevy::input::ButtonInput;
use bevy::prelude::{
    default, in_state, Circle, Color, ColorMaterial, Commands, Component, Entity,
    IntoSystemConfigs, KeyCode, Mesh, OnEnter, Query, Res, ResMut, Time, Transform, With,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::game::wall;
use crate::state::GameState;

pub const PLAYER_RADIUS: f32 = 5.;
const PLAYER_SPEED: f32 = 300.;

impl bevy::app::Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Running),
            (despawn_player, spawn_player).chain(),
        )
        .add_systems(
            bevy::app::FixedUpdate,
            move_player.run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Component)]
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bundle = MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle::new(PLAYER_RADIUS))),
        material: materials.add(Color::rgb(255., 255., 255.)),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    };
    cmd.spawn((Player, bundle));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let (mut delta_x, mut delta_y) = (0., 0.);
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        delta_y += 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        delta_y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        delta_x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        delta_x += 1.;
    }
    let (distance_x, distance_y) = (
        delta_x * PLAYER_SPEED * time.delta_seconds(),
        delta_y * PLAYER_SPEED * time.delta_seconds(),
    );
    let mut transform = player.get_single_mut().unwrap();
    let (new_x, new_y) = (
        transform.translation.x + distance_x,
        transform.translation.y + distance_y,
    );
    transform.translation.x = new_x.clamp(
        wall::LEFT_WALL_POS + PLAYER_RADIUS,
        wall::RIGHT_WALL_POS - PLAYER_RADIUS,
    );
    transform.translation.y = new_y.clamp(
        wall::BOTTOM_WALL_POS + PLAYER_RADIUS,
        wall::TOP_WALL_POS - PLAYER_RADIUS,
    );
}

fn despawn_player(mut cmd: Commands, query: Query<Entity, With<Player>>) {
    if let Ok(player) = query.get_single() {
        cmd.entity(player).despawn();
    }
}
