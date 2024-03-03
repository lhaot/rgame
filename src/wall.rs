use bevy::app::App;
use bevy::math::{vec2, Vec2};
use bevy::prelude::{Color, Commands, Component, Transform};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;

use crate::wall::WallLocation::{Bottom, Left, Right, Top};

pub const WALL_LEN: f32 = 400.;
pub const LEFT_WALL_POS: f32 = -WALL_LEN / 2.;
pub const RIGHT_WALL_POS: f32 = WALL_LEN / 2.;
pub const TOP_WALL_POS: f32 = WALL_LEN / 2.;
pub const BOTTOM_WALL_POS: f32 = -WALL_LEN / 2.;
pub const WALL_THICKNESS: f32 = 1.;
const WALL_COLOR: Color = Color::rgb(128., 128., 128.);

#[derive(Component)]
pub struct WallPlugin;

impl bevy::app::Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(bevy::app::Startup, spawn_wall);
    }
}

#[derive(Component)]
struct Wall;

pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    #[inline]
    pub fn values() -> Vec<WallLocation> {
        vec![Left, Right, Bottom, Top]
    }

    pub fn begin_pos(&self) -> Vec2 {
        match self {
            Left => vec2(LEFT_WALL_POS, BOTTOM_WALL_POS),
            Right => vec2(RIGHT_WALL_POS, BOTTOM_WALL_POS),
            Bottom => vec2(LEFT_WALL_POS, BOTTOM_WALL_POS),
            Top => vec2(LEFT_WALL_POS, TOP_WALL_POS),
        }
    }

    pub fn go_on_wall(&self, distance: f32) -> Vec2 {
        let mut pos = self.begin_pos();
        match self {
            Left | Right => pos.y += distance,
            Bottom | Top => pos.x += distance,
        }
        pos
    }
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            Left => Vec2::new(LEFT_WALL_POS, 0.),
            Right => Vec2::new(RIGHT_WALL_POS, 0.),
            Bottom => Vec2::new(0., BOTTOM_WALL_POS),
            Top => Vec2::new(0., TOP_WALL_POS),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL_POS - BOTTOM_WALL_POS;
        let arena_width = RIGHT_WALL_POS - LEFT_WALL_POS;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            Left | Right => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            Bottom | Top => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
        }
    }
}

#[inline]
pub fn is_out_of_wall(x: f32, y: f32) -> bool {
    !is_in_or_on_wall(x, y)
}

#[inline]
pub fn is_in_or_on_wall(x: f32, y: f32) -> bool {
    LEFT_WALL_POS <= x && x <= RIGHT_WALL_POS && BOTTOM_WALL_POS <= y && y <= TOP_WALL_POS
}

fn spawn_wall(mut cmd: Commands) {
    cmd.spawn((Wall, wall_bundle(Left)));
    cmd.spawn((Wall, wall_bundle(Right)));
    cmd.spawn((Wall, wall_bundle(Bottom)));
    cmd.spawn((Wall, wall_bundle(Top)));
}

fn wall_bundle(location: WallLocation) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: WALL_COLOR,
            ..default()
        },
        transform: Transform {
            translation: location.position().extend(0.),
            scale: location.size().extend(1.),
            ..default()
        },
        ..default()
    }
}
