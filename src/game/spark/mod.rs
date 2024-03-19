use bevy::app::{App, FixedUpdate};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Deref, DerefMut, Entity, in_state, IntoSystemConfigs, NextState, OnEnter, Plugin, Query, States, With};
use rand::Rng;

use rgame::Values;

use crate::state::GameState;

mod auto;

#[derive(Values, Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Spark {
    Auto,
}

#[derive(States, Default, Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum SparkState {
    #[default]
    Idle,
    Sparking(Spark),
}

#[derive(Component)]
pub struct SparkPlugin;

#[derive(Component)]
pub(crate) struct Enemy;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

impl Plugin for SparkPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SparkState>()
            .insert_resource(NextState(Some(SparkState::Idle)))
            .add_systems(OnEnter(GameState::Running), despawn_enemy)
            .add_systems(
                FixedUpdate,
                into_rand_spark
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(SparkState::Idle)),
            )
            .add_plugins(auto::Auto);
    }
}

fn despawn_enemy(
    mut cmd: Commands,
    enemies: Query<Entity, With<Enemy>>,
) {
    for enemy in enemies.iter() {
        cmd.entity(enemy).despawn();
    }
}

fn into_rand_spark(mut cmd: Commands) {
    let i = rand::thread_rng().gen_range(0..Spark::values.len());
    let spark = Spark::values[i].clone();
    cmd.insert_resource(NextState(Some(SparkState::Sparking(spark))));
}
