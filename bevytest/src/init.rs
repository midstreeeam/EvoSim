use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

// use crate::{
//     config::*
// };

use crate::{blob::*, wall::*};

pub fn world_setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(TopWallBundle::default());
    commands.spawn(BottomWallBundle::default());
    commands.spawn(LeftWallBundle::default());
    commands.spawn(RightWallBundle::default());
}

pub fn graphic_setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_blobs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(BlobBundle::new(&mut meshes,&mut materials));
    commands.spawn(BlobBundle::new(&mut meshes,&mut materials));
}
