use bevy::prelude::*;

use super::{export::export, import::load_blobs};

pub struct EvoIO;

impl Plugin for EvoIO {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, load_blobs)
        .add_systems(Update, export)
        ;
    }
}