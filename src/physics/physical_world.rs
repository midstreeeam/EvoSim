use bevy::prelude::*;

use crate::physics::world::setup_walls;
use crate::physics::rules::*;

pub struct PhysiWorld;

impl Plugin for PhysiWorld {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Startup,(
                setup_walls,
                setup_gravity,
                // apply_forces
        )).add_systems(
            Update,viscosity
        );
    }
}