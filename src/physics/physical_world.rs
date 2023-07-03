use bevy::prelude::*;

use crate::physics::world::setup_walls;
use crate::physics::rules::*;

pub struct PhysiWorld;

impl Plugin for PhysiWorld {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_walls)
        .add_startup_system(setup_gravity)
        // .add_system(apply_forces)
        ;
    }
}