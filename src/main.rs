#![allow(dead_code)]

mod blob;
mod brain;
mod componet;
mod consts;
mod contorl;
mod graphics;
mod io;
mod physics;
mod mutate;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use blob::geno_blob_builder::{BlobGeno, GenoBlobBuilder};
use brain::resource::BevyBlockNeurons;
// use consts::THREAD_COUNT;
use contorl::contorl::BlobContorlPlugin;
use graphics::*;
use io::evoio::EvoIOPlugin;
use physics::physical_world::PhysiWorldPlugin;
use mutate::mutate::MutatePlugin;

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
            // raiper
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            // cost
            PhysiWorldPlugin, // init physical world
            EvoGraphicsPlugin, // vsync and camera
            EvoIOPlugin, // import and export
            MutatePlugin, // mutation contorl
            BlobContorlPlugin // update blob each frame
        ))
        .add_systems(Startup, setup_test)
        .init_resource::<BevyBlockNeurons>()
        .run();
}

pub fn setup_test(commands: Commands, mut bbns: ResMut<BevyBlockNeurons>) {
    let mut builder = GenoBlobBuilder::from_commands(commands, &mut bbns.nnvec);
    // let mut geno = BlobGeno::new_rand();
    // builder.build(&mut geno, [-500.0, 0.0]);
    // println!("{:#?}",geno);

    for i in -2..2 {
        for j in -2..2 {
            builder.build(&mut BlobGeno::new_rand(), [1500.0 * i as f32, 1500.0 * j as f32]);
        }
    }
}