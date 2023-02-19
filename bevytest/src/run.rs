use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    config::*,
    // componets::*,
    init::*
};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(30.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration{
            // gravity: GRAVITY,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))

        // init
        .add_startup_system_set(
            SystemSet::new()
                .with_system(graphic_setup)
                .with_system(world_setup)
                .with_system(spawn_blobs)
        )

        .add_system_set(
            SystemSet::new()
            //.with_system(print_ball_altitude)
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

// fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
//     for transform in positions.iter() {
//         println!("Ball altitude: {}", transform.translation.y);
//     }
// }