#![allow(dead_code)]

mod blob;
mod brain;
mod consts;
mod graphics;
mod physics;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use blob::geno_blob_builder::{BlobGeno, GenoBlobBuilder};
use brain::resource::BevyBlockNeurons;
use graphics::*;
use physics::physical_world;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

// TODO: Headless mode causing panic
// TODO: Not all cores are fully tuilized
fn main() {
    App::new()
        .add_plugins((
            // defualt
            DefaultPlugins,
            // // no renderer
            // DefaultPlugins.set(RenderPlugin {
            //     wgpu_settings: WgpuSettings {
            //         backends: None,
            //         ..default()
            //     }
            // }),

            // log frame rate
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            // raiper
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            // cost
            physical_world::PhysiWorld,
            Graphics,
        ))
        .add_systems(Startup, setup_test)
        // .init_resource::<BevyBlockNeurons>()
        // .add_systems(Update, res_test)
        .run();
}

fn setup_test(commands: Commands) {
    let mut builder = GenoBlobBuilder::from_commands(commands);
    builder.build(&BlobGeno::new_rand(), [0.0, 0.0]);
}

fn res_test(res: Res<BevyBlockNeurons>) {
    res.nnvec.first().unwrap().thread_test();
}

/// Generate 100 random blobs.
/// Pressure test for Rapier
fn pressure_test(commands: Commands) {
    let mut builder = GenoBlobBuilder::from_commands(commands);
    for i in -5..5 {
        for j in -5..5 {
            builder.build(&BlobGeno::new_rand(), [700.0 * i as f32, 700.0 * j as f32]);
        }
    }
}
