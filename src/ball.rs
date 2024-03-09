use std::ops::DivAssign;

use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::math::{vec2, vec3, Vec2, Vec3};
use bevy::prelude::{
    in_state, Circle, Color, Commands, Component, Deref, DerefMut, Entity, IntoSystemConfigs, Mesh,
    Query, Res, ResMut, Resource, Transform, With,
};
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::time::{Time, Timer, TimerMode};
use bevy::utils::default;
use rand::Rng;

use crate::player::Player;
use crate::state::GameState;
use crate::wall;
use crate::wall::{is_out_of_wall, WALL_LEN};

const GEN_NUM_PER_TIME: i32 = 2;
const BALL_BASE_SPEED: f32 = 150.;
const BALL_FLUCTUATE_RANGE: f32 = 250.;
pub const BALL_RADIUS: f32 = 3.;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1., TimerMode::Repeating)))
            .add_systems(
                bevy::app::FixedUpdate,
                (apply_velocity, spawn_enemy, despawn_if_out_of_wall)
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Component)]
pub struct EnemyPlugin;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_enemy(
    mut cmd: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    player_transform: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for _ in 0..GEN_NUM_PER_TIME {
        let pos = rand_spawn_pos();
        let bundle = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(BALL_RADIUS))),
            material: materials.add(Color::rgb(255., 0., 0.)),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..default()
        };

        let player_translation = player_transform.get_single().unwrap().translation;
        let velocity = velocity_to_player(pos, player_translation);
        cmd.spawn((Ball, bundle, velocity));
    }
}

fn rand_spawn_pos() -> Vec3 {
    let walls = wall::WallLocation::values();
    let i = rand::thread_rng().gen_range(0..walls.len());
    let wall = &walls[i];
    let pos = wall.go_on_wall(rand::random::<f32>() * WALL_LEN);
    vec3(pos.x, pos.y, 1.)
}

fn velocity_to_player(pos: Vec3, player_translation: Vec3) -> Velocity {
    let (mut vx, mut vy) = (player_translation.x - pos.x, player_translation.y - pos.y);
    let distance = (vx.powi(2) + vy.powi(2)).sqrt();
    vx.div_assign(distance);
    vy.div_assign(distance);
    (vx, vy) = (
        vx * ((0.5 - rand::random::<f32>()) * BALL_FLUCTUATE_RANGE + BALL_BASE_SPEED),
        vy * ((0.5 - rand::random::<f32>()) * BALL_FLUCTUATE_RANGE + BALL_BASE_SPEED),
    );
    Velocity(vec2(vx, vy))
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn despawn_if_out_of_wall(mut cmd: Commands, query: Query<(Entity, &Transform), With<Ball>>) {
    for (entity, transform) in &query {
        if is_out_of_wall(transform.translation.x, transform.translation.y) {
            cmd.entity(entity).despawn();
        }
    }
}
