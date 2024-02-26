use bevy::app::App;
use bevy::asset::Assets;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Circle, Color, Commands, Component, default, KeyCode, Mesh, Query, Res, ResMut, Transform, With,
};
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::time::Time;

#[derive(Component)]
pub struct PlayerPlugin;

impl bevy::app::Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(bevy::app::Startup, spawn_player);
        app.add_systems(bevy::app::FixedUpdate, move_player);
    }
}

#[derive(Component)]
struct Player;

const PLAYER_SPEED: f32 = 200.;

fn spawn_player(
    mut cmd: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bundle = MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshs.add(Circle::new(10.0))),
        material: materials.add(Color::rgb(255., 255., 255.)),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    };
    cmd.spawn((Player, bundle));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
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
    let mut player_transform = query.get_single_mut().unwrap();
    player_transform.translation.x += distance_x;
    player_transform.translation.y += distance_y;
}
