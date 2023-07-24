use bevy::prelude::*;

use crate::contorl::block_action;

use super::{export::export, import::{load_blobs, clean}};

pub struct EvoIO;

impl Plugin for EvoIO {
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