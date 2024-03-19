use std::ops::DivAssign;

use bevy::math::{vec2, vec3};
use bevy::prelude::{
    App, Assets, Circle, Color, ColorMaterial, Commands, Component, default, Entity, FixedUpdate,
    in_state, IntoSystemConfigs, Mesh, NextState, Plugin, Query, Res, ResMut, Resource, Time,
    Timer, TimerMode, Transform, Vec3, With,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::Rng;

use crate::game::{player, wall};
use crate::game::player::Player;
use crate::game::spark::{Enemy, Spark, SparkState, Velocity};
use crate::game::wall::{is_out_of_wall, WALL_LEN};
use crate::state::GameState;

#[derive(Component)]
pub(super) struct Auto;

#[derive(Resource)]
struct DurationTimer(Timer);

#[derive(Resource)]
struct CoolDownTimer(Timer);

impl Plugin for Auto {
    fn build(&self, app: &mut App) {
        app.insert_resource(DurationTimer(Timer::from_seconds(10., TimerMode::Repeating)))
            .insert_resource(CoolDownTimer(Timer::from_seconds(1., TimerMode::Repeating)))
            .add_systems(
                FixedUpdate,
                (spark, apply_velocity, despawn_if_out_of_wall, pause_if_collision, end)
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(SparkState::Sparking(Spark::Auto))),
            );
    }
}

const BALL_RADIUS: f32 = 3.;
const GEN_NUM_PER_TIME: i32 = 3;
const BALL_BASE_SPEED: f32 = 150.;
const BALL_FLUCTUATE_RANGE: f32 = 250.;

fn spark(
    mut cmd: Commands,
    time: Res<Time>,
    duration_timer: ResMut<DurationTimer>,
    mut cool_down_timer: ResMut<CoolDownTimer>,
    player_transform: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !cool_down_timer.0.tick(time.delta()).finished() {
        return;
    }
    let d = duration_timer.0.elapsed().as_secs() as i32 / 3;
    for _ in 0..(GEN_NUM_PER_TIME + d) {
        // gen pos
        let pos = gen_pos();
        // gen unit velocity
        let player_pos = player_transform.get_single().unwrap().translation;
        let unit_velocity = gen_velocity(pos, player_pos);
        // build bundle
        let bundle = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(BALL_RADIUS))),
            material: materials.add(Color::rgb(255., 0., 0.)),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..default()
        };
        // spawn
        cmd.spawn((Enemy, bundle, unit_velocity));
    }
}

fn gen_velocity(pos: Vec3, player_pos: Vec3) -> Velocity {
    let (mut vx, mut vy) = (player_pos.x - pos.x, player_pos.y - pos.y);
    let distance = (vx.powi(2) + vy.powi(2)).sqrt();
    vx.div_assign(distance);
    vy.div_assign(distance);
    (vx, vy) = (
        vx * ((0.5 - rand::random::<f32>()) * BALL_FLUCTUATE_RANGE + BALL_BASE_SPEED),
        vy * ((0.5 - rand::random::<f32>()) * BALL_FLUCTUATE_RANGE + BALL_BASE_SPEED),
    );
    Velocity(vec2(vx, vy))
}

fn gen_pos() -> Vec3 {
    let walls = wall::WallLocation::values();
    let i = rand::thread_rng().gen_range(0..walls.len());
    let wall = &walls[i];
    let pos = wall.go_on_wall(rand::random::<f32>() * WALL_LEN);
    vec3(pos.x, pos.y, 1.)
}

fn apply_velocity(mut enemy: Query<(&mut Transform, &Velocity), With<Enemy>>, time: Res<Time>) {
    for (mut transform, velocity) in &mut enemy {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn despawn_if_out_of_wall(
    mut cmd: Commands,
    query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity, transform) in &query {
        if is_out_of_wall(transform.translation.x, transform.translation.y) {
            cmd.entity(entity).despawn();
        }
    }
}

fn pause_if_collision(
    mut cmd: Commands,
    player_transform_query: Query<&Transform, With<Player>>,
    ball_transforms_query: Query<&Transform, With<Enemy>>,
) {
    let collision_powi2 = (player::PLAYER_RADIUS + BALL_RADIUS).powi(2);
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

fn end(mut cmd: Commands, time: Res<Time>, mut duration_timer: ResMut<DurationTimer>) {
    if duration_timer.0.tick(time.delta()).finished() {
        cmd.insert_resource(NextState(Some(SparkState::Idle)));
    }
}
