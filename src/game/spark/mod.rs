use bevy::app::{App, FixedUpdate};
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Component, Deref, DerefMut, in_state, IntoSystemConfigs,
    NextState, Plugin, States,
};
use rand::Rng;

use rgame::Values;

use crate::state::GameState;

mod auto;

pub const BALL_RADIUS: f32 = 3.;

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
            .add_systems(
                FixedUpdate,
                into_rand_spark
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(SparkState::Idle)),
            )
            .add_plugins(auto::Auto);
    }
}

fn into_rand_spark(mut cmd: Commands) {
    let i = rand::thread_rng().gen_range(0..Spark::values.len());
    let spark = Spark::values[i].clone();
    cmd.insert_resource(NextState(Some(SparkState::Sparking(spark))));
}
