#![allow(dead_code)]

mod blob;
mod brain;
mod componet;
mod consts;
mod contorl;
mod graphics;
mod io;
mod mutate;
mod physics;

#[macro_use]
mod logger;

use bevy::prelude::*;

use brain::resource::BevyBlockNeurons;
use contorl::contorl::BlobContorlPlugin;
use graphics::*;
use io::evoio::EvoIOPlugin;
use mutate::mutate::MutatePlugin;
use physics::physical_world::PhysiWorldPlugin;

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

            // custom
            PhysiWorldPlugin,  // init physical world
            EvoGraphicsPlugin, // vsync and camera
            EvoIOPlugin,       // import and export
            MutatePlugin,      // mutation contorl
            BlobContorlPlugin, // update blob each frame
        ))
        .init_resource::<BevyBlockNeurons>()
        .run();
}
