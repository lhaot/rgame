use std::ops::AddAssign;
use std::time::Duration;

use bevy::app::App;
use bevy::prelude::{
    Commands, Component, Deref, DerefMut, FixedUpdate, in_state, IntoSystemConfigs, OnEnter, Query,
    Res, ResMut, Resource, Startup, Text, TextBundle, TextSection, TextStyle, Time, With,
};
use bevy::utils::default;

use crate::state::GameState;

#[derive(Component)]
pub(crate) struct UiPlugin;

impl bevy::app::Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(OnEnter(GameState::Running), reset_ui)
            .add_systems(FixedUpdate, update_ui.run_if(in_state(GameState::Running)));
    }
}

const TIME_FONT_SIZE: f32 = 40.;

#[derive(Component)]
struct TimeBoardUi;

#[derive(Resource, Deref, DerefMut)]
struct RoundDuration(Duration);

fn setup_ui(mut cmd: Commands) {
    // setup time board ui
    cmd.spawn((
        TimeBoardUi,
        TextBundle::from_sections([
            TextSection {
                value: String::from("Time: "),
                style: TextStyle {
                    font_size: TIME_FONT_SIZE,
                    ..default()
                },
            },
            TextSection {
                value: fmt_duration(&Duration::default()),
                style: TextStyle {
                    font_size: TIME_FONT_SIZE,
                    ..default()
                },
            },
        ]),
    ));
    // insert round duration
    cmd.insert_resource(RoundDuration(Duration::default()));
}

fn update_ui(
    time: Res<Time>,
    mut duration: ResMut<RoundDuration>,
    mut query: Query<&mut Text, With<TimeBoardUi>>,
) {
    duration.add_assign(time.delta());
    let mut time_board_txt = query.get_single_mut().unwrap();
    time_board_txt.sections[1].value = fmt_duration(&duration.0)
}

fn reset_ui(
    mut txt_query: Query<&mut Text, With<TimeBoardUi>>,
    mut duration: ResMut<RoundDuration>,
) {
    duration.0 = Duration::default();
    txt_query.get_single_mut().unwrap().sections[1].value = fmt_duration(&duration.0);
}

fn fmt_duration(duration: &Duration) -> String {
    let millis = duration.as_millis() % 1000;
    let secs = duration.as_secs() % 60;
    let mins = duration.as_secs() / 60;
    format!("{:0>2}:{:0>2}:{:0>3}", mins, secs, millis)
}
