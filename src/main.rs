#![allow(dead_code)]


mod physics;
mod blob;
mod graphics;
mod consts;

use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::*
};

use physics::physical_world;
use graphics::*;
use blob::{block::PhysiBlockBundle, blob_builder::BlobBuilder};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(physical_world::PhysiWorld)
        .add_plugin(Graphics)
        .add_startup_system(setup_test)
        .run();
}

fn setup_test(
    commands: Commands,
) {
        
    let mut bb = BlobBuilder::from_commands(commands);

    let body = PhysiBlockBundle::from_xy_dx_dy(
            0.0, 0.0, 25.0, 50.0
        ).with_density(1.0);

    let body2 = PhysiBlockBundle::from_xy_dx_dy(
            -100.0, 0.0, 25.0, 50.0
        ).with_density(1.0);
    
    bb.set_color(Color::RED).create_first(body, ()).add_to_bottom(
        50.0, 50.0, Some(155f32.to_radians()),None,()
    ).add_to_right(
        50.0, 50.0, Some(155f32.to_radians()),None,()
    ).add_to_right(
        30.0, 50.0, Some(155f32.to_radians()),None,()
    ).left().add_to_bottom(
        30.0, 50.0, Some(155f32.to_radians()),None,()
    );

    // create new blob
    bb.clean().set_color(Color::LIME_GREEN).create_first(body2,()).add_to_top(
        50.0, 50.0, Some(155f32.to_radians()),None,()
    );

    // println!("{:#?}",bb.blocks);

}