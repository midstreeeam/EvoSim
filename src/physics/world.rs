use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{consts::*, componet::ColliderFlag};

/// wall flag, different from `ColliderFlag`
#[derive(Component)]
pub struct Wall;

pub fn setup_walls(mut commands: Commands) {

    let mut half_window_width = WORLD_WIDTH_SWIM / 2.0;
    let mut half_window_height = WORLD_HEIGHT_SWIM /2.0;
    
    if TRAINING_MODE == "walk" {
        half_window_width = WORLD_WIDTH_WALK / 2.0;
        half_window_height = WORLD_HEIGHT_WALK /2.0;
    }


    // Left wall
    commands.spawn((
        Collider::cuboid(1.0, half_window_height),
        TransformBundle::from_transform(Transform::from_xyz(-half_window_width, 0.0, 0.0)),
        ColliderFlag::WALL,
        Wall
    ));

    // Right wall
    commands.spawn((
        Collider::cuboid(1.0, half_window_height),
        TransformBundle::from_transform(Transform::from_xyz(half_window_width, 0.0, 0.0)),
        ColliderFlag::WALL,
        Wall
    ));

    // Top wall
    commands.spawn((
        Collider::cuboid(half_window_width, 1.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, half_window_height, 0.0)),
        ColliderFlag::WALL,
        Wall
    ));

    // Bottom wall
    commands.spawn((
        Collider::cuboid(half_window_width, 1.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, -half_window_height, 0.0)),
        ColliderFlag::WALL,
        Wall
    ));
}
