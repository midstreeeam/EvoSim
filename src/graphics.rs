use bevy::{prelude::*, window::PresentMode};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_rapier2d::prelude::{RapierConfiguration, TimestepMode};

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use crate::consts::{AUTO_NO_VSYNC_KEYCODE, RAPIER_DT, RAPIER_SUBSTEPS};

#[derive(Component)]
pub struct MainCamera;
pub struct EvoGraphicsPlugin;

impl Plugin for EvoGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_graphics)
            .add_plugins((
                PanCamPlugin::default(),
                // // log frame rate
                // LogDiagnosticsPlugin::default(),
                // FrameTimeDiagnosticsPlugin::default(),
            ))
            .add_systems(Update, toggle_vsync)
            // using Fixed timestep so that the simulation can speed up
            .insert_resource(RapierConfiguration {
                timestep_mode: TimestepMode::Fixed {
                    dt: RAPIER_DT,
                    substeps: RAPIER_SUBSTEPS,
                },
                ..default()
            });
    }

    
}

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PanCam::default(), MainCamera));
}

/// This system toggles the vsync mode when pressing the button.
fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(AUTO_NO_VSYNC_KEYCODE) {
        let mut window = windows.single_mut();

        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
}
