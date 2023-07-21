use bevy::prelude::*;

use super::export::export;

pub struct EvoIO;

impl Plugin for EvoIO {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, export)
        ;
    }
}