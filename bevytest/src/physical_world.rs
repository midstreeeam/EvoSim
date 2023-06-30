use bevy::{
    prelude::*,
    window::PrimaryWindow
};
use bevy_rapier2d::prelude::*;

pub struct PhysiWorld;

impl Plugin for PhysiWorld {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_walls)
        .add_startup_system(setup_gravity);
    }
}


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

pub fn setup_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}
