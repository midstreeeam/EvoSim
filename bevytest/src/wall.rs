use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::*;

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct TopWallBundle {
    pub wall: Wall,
    pub top_collider: Collider,
    pub top_transform_bundle: TransformBundle,

}

impl Default for TopWallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            top_collider: Collider::cuboid(
                (WALL_RIGHT-WALL_LEFT)/2.0,
                WALL_THICKNESS),
            top_transform_bundle: TransformBundle::from(
                Transform::from_xyz(0.0, WALL_TOP-(WALL_TOP+WALL_BOTTOM)/2.0, 0.0))
        }
    }
}


#[derive(Bundle)]
pub struct BottomWallBundle {
    pub wall: Wall,
    pub bottom_collider: Collider,
    pub bottom_transform_bundle: TransformBundle,

}

impl Default for BottomWallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            bottom_collider: Collider::cuboid(
                (WALL_RIGHT-WALL_LEFT)/2.0,
                WALL_THICKNESS),
            bottom_transform_bundle: TransformBundle::from(
                Transform::from_xyz(0.0, WALL_BOTTOM-(WALL_TOP+WALL_BOTTOM)/2.0, 0.0))
        }
    }
}


#[derive(Bundle)]
pub struct LeftWallBundle {
    pub wall: Wall,
    pub left_collider: Collider,
    pub left_transform_bundle: TransformBundle,

}

impl Default for LeftWallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            left_collider: Collider::cuboid(
                WALL_THICKNESS,
                (WALL_TOP-WALL_BOTTOM)/2.0),
            left_transform_bundle: TransformBundle::from(
                Transform::from_xyz(WALL_LEFT, 0.0, 0.0)),
        }
    }
}


#[derive(Bundle)]
pub struct RightWallBundle {
    pub wall: Wall,
    pub right_collider: Collider,
    pub right_transform_bundle: TransformBundle,
}

impl Default for RightWallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            right_collider: Collider::cuboid(
                WALL_THICKNESS,
                (WALL_TOP-WALL_BOTTOM)/2.0),
            right_transform_bundle: TransformBundle::from(
                Transform::from_xyz(WALL_RIGHT, 0.0, 0.0))
        }
    }
}