use bevy::{
    prelude::*,
    window::PrimaryWindow
};
use bevy_rapier2d::prelude::*;


#[derive(Component)]
pub struct Wall{}

pub fn setup_walls(mut commands: Commands,window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let half_window_width = window.width() / 2.0;
    let half_window_height = window.height() / 2.0;

    // Left wall
    commands.spawn((
        Collider::cuboid(1.0, half_window_height),
        TransformBundle::from_transform(Transform::from_xyz(-half_window_width, 0.0, 0.0)),
        Wall{}
    ));

    // Right wall
    commands.spawn((
        Collider::cuboid(1.0, half_window_height),
        TransformBundle::from_transform(Transform::from_xyz(half_window_width, 0.0, 0.0)),
        Wall{}
    ));

    // Top wall
    commands.spawn((
        Collider::cuboid(half_window_width, 1.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, half_window_height, 0.0)),
        Wall{}
    ));

    // Bottom wall
    commands.spawn((
        Collider::cuboid(half_window_width, 1.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, -half_window_height, 0.0))
    ));
}
