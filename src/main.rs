#![allow(dead_code)]


mod physics;
mod blob;
mod graphics;
mod consts;
mod brain;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use brain::resource::BevyBlockNeurons;
use physics::physical_world;
use graphics::*;
use blob::{block::PhysiBlockBundle, blob_builder::BlobBuilder, geno_blob_builder::{BlobGeno, GenoBlobBuilder}};


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
        .add_systems(Startup,setup_test)
        // .init_resource::<BevyBlockNeurons>()
        // .add_systems(Update, res_test)
        .run();
}

fn setup_test(
    commands: Commands,
) {
        
    // let mut bb = BlobBuilder::from_commands(commands);

    // let body = PhysiBlockBundle::from_xy_dx_dy(
    //         0.0, 0.0, 25.0, 50.0
    //     ).with_density(1.0);

    // let body2 = PhysiBlockBundle::from_xy_dx_dy(
    //         -100.0, 0.0, 25.0, 50.0
    //     ).with_density(1.0);
    
    // bb.set_color(Color::RED).create_first(body, ()).add_to_bottom(
    //     50.0, 50.0, Some(155f32.to_radians()),None,()
    // ).add_to_right(
    //     50.0, 50.0, Some(155f32.to_radians()),None,()
    // ).add_to_right(
    //     30.0, 50.0, Some(155f32.to_radians()),None,()
    // ).left().add_to_bottom(
    //     30.0, 50.0, Some(155f32.to_radians()),None,()
    // );

    // // create new blob
    // bb.clean().set_color(Color::LIME_GREEN).create_first(body2,()).add_to_top(
    //     50.0, 50.0, Some(155f32.to_radians()),None,()
    // );

    // // println!("{:#?}",bb.blocks);

    let mut builder = GenoBlobBuilder::from_commands(commands);
    builder.build(&BlobGeno::new_rand(), [0.0,0.0]);


}


fn res_test(res: Res<BevyBlockNeurons>){
    res.nnvec.first().unwrap().thread_test();
}



/// Generate 100 random blobs.
/// Pressure test for Rapier
fn pressure_test(
    commands: Commands,
) {
    let mut builder = GenoBlobBuilder::from_commands(commands);
    for i in -5..5{
        for j in -5..5{
            builder.build(&BlobGeno::new_rand(), [700.0*i as f32, 700.0*j as f32]);
        }
    }
}