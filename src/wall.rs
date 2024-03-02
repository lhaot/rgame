use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::{Color, Commands, Component, Transform};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;

pub const LEFT_WALL: f32 = -200.;
pub const RIGHT_WALL: f32 = 200.;
pub const TOP_WALL: f32 = 200.;
pub const BOTTOM_WALL: f32 = -200.;
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

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

pub fn is_out_of_wall(x: f32, y: f32) -> bool {
    !is_in_wall(x, y)
}

pub fn is_in_wall(x: f32, y: f32) -> bool {
    LEFT_WALL < x && x < RIGHT_WALL && BOTTOM_WALL < y && y < TOP_WALL
}

fn spawn_wall(mut cmd: Commands) {
    cmd.spawn((Wall, wall_bundle(WallLocation::Left)));
    cmd.spawn((Wall, wall_bundle(WallLocation::Right)));
    cmd.spawn((Wall, wall_bundle(WallLocation::Bottom)));
    cmd.spawn((Wall, wall_bundle(WallLocation::Top)));
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
