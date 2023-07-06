use bevy::prelude::*;
use bevy_pancam::{PanCamPlugin, PanCam};

#[derive(Component)]
pub struct MainCamera;
pub struct Graphics;

impl Plugin for Graphics {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_graphics)
        .add_plugin(PanCamPlugin::default())
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