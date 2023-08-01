use bevy::prelude::*;

use crate::contorl::update::block_action;

use super::{export::export, import::{load_blobs, clean}};

pub struct EvoIOPlugin;

impl Plugin for EvoIOPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
            export,
            clean.after(block_action),
            load_blobs.after(clean),
        ))
        ;
    }
}