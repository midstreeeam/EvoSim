use bevy::{prelude::*, window::PresentMode};
use bevy_pancam::{PanCamPlugin, PanCam};
use bevy_rapier2d::prelude::{RapierConfiguration, TimestepMode};

use crate::consts::{RAPIER_DT, RAPIER_SUBSTEPS};

#[derive(Component)]
pub struct MainCamera;
pub struct Graphics;

impl Plugin for Graphics {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_graphics)
        .add_plugin(PanCamPlugin::default())
        .add_system(toggle_vsync)

        // using Fixed timestep so that the simulation can speed up
        .insert_resource(RapierConfiguration{
            timestep_mode: TimestepMode::Fixed {
                dt: RAPIER_DT, substeps: RAPIER_SUBSTEPS
            },
            ..default()
        })
        ;
    }
}

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        PanCam::default(),
        MainCamera
    ));
}

/// This system toggles the vsync mode when pressing the button V.
/// You'll see fps increase displayed in the console.
fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::V) {
        let mut window = windows.single_mut();

        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
}