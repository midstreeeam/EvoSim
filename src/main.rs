#![allow(dead_code)]

mod blob;
mod brain;
mod componet;
mod consts;
mod contorl;
mod graphics;
mod physics;
mod io;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use blob::geno_blob_builder::{BlobGeno, GenoBlobBuilder};
use brain::resource::BevyBlockNeurons;
// use consts::THREAD_COUNT;
use contorl::{block_action, update_blob_info, update_joint_info};
use graphics::*;
use io::evoio::EvoIO;
use physics::physical_world;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

// TODO: Inconsistant usize and u32 usage
// TODO: Headless mode causing panic
// TODO: Not all cores are fully tuilized
fn main() {
    App::new()
        .add_plugins((
            // defualt
            DefaultPlugins,
            // // set thread count
            // DefaultPlugins.set(
            //     TaskPoolPlugin{
            //         task_pool_options: TaskPoolOptions::with_num_threads(THREAD_COUNT)
            //     }
            // ),
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
            EvoIO
        ))
        .add_systems(Startup, setup_test)
        .init_resource::<BevyBlockNeurons>()
        .add_systems(Update, (
            block_action, 
            update_joint_info, 
            update_blob_info,
            // test
        ))
        .run();
}

pub fn setup_test(commands: Commands, mut bbns: ResMut<BevyBlockNeurons>) {
    let mut builder = GenoBlobBuilder::from_commands(commands, &mut bbns.nnvec);
    // builder.build(&BlobGeno::new_rand(), [0.0, 0.0]);

    for i in -2..2 {
        for j in -2..2 {
            builder.build(&BlobGeno::new_rand(), [700.0 * i as f32, 700.0 * j as f32]);
        }
    }
}

fn test(q: Query<&BlobGeno>) {
    for _ in q.iter(){
        // println!("{:#?}",i);
    }
}

/// Generate 100 random blobs.
/// Pressure test for Rapier
fn pressure_test(commands: Commands, mut bbns: ResMut<BevyBlockNeurons>) {
    let mut builder = GenoBlobBuilder::from_commands(commands, &mut bbns.nnvec);
    for i in -5..5 {
        for j in -5..5 {
            builder.build(&BlobGeno::new_rand(), [700.0 * i as f32, 700.0 * j as f32]);
        }
    }
}
