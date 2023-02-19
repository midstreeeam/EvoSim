use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::*;

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct TopWallBundle {
    pub wall: Wall,
    pub sprite_bundle: SpriteBundle,
    pub top_collider: Collider,
}

impl Default for TopWallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(Vec2::new(WALL_X*2., WALL_THICKNESS*2.)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, WALL_Y, 0.0),
                ..default()
            },
            top_collider: Collider::cuboid(WALL_X, WALL_THICKNESS),
        }
    }
}

#[derive(Bundle)]
pub struct BottomWallBundle {
    pub wall: Wall,
    pub sprite_bundle: SpriteBundle,
    pub bottom_collider: Collider,
}

impl Default for BottomWallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(Vec2::new(WALL_X*2., WALL_THICKNESS*2.)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, -WALL_Y, 0.0),
                ..default()
            },
            bottom_collider: Collider::cuboid(WALL_X, WALL_THICKNESS),
        }
    }
}

#[derive(Bundle)]
pub struct LeftWallBundle {
    pub wall: Wall,
    pub sprite_bundle: SpriteBundle,
    pub left_collider: Collider,
}

impl Default for LeftWallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(Vec2::new(WALL_THICKNESS*2., WALL_Y*2.)),
                    ..default()
                },
                transform: Transform::from_xyz(-WALL_X, 0.0, 0.0),
                ..default()
            },
            left_collider: Collider::cuboid(WALL_THICKNESS, WALL_Y),
        }
    }
}

#[derive(Bundle)]
pub struct RightWallBundle {
    pub wall: Wall,
    pub sprite_bundle: SpriteBundle,
    pub right_collider: Collider,
}

impl Default for RightWallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(Vec2::new(WALL_THICKNESS*2., WALL_Y*2.)),
                    ..default()
                },
                transform: Transform::from_xyz(WALL_X, 0.0, 0.0),
                ..default()
            },
            right_collider: Collider::cuboid(WALL_THICKNESS, WALL_Y),
        }
    }
}
