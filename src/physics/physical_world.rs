use bevy::prelude::*;

use crate::physics::rules::*;
use crate::physics::world::setup_walls;

pub struct PhysiWorldPlugin;

impl Plugin for PhysiWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_walls,
                setup_gravity,
                // apply_forces
            ),
        )
        .add_systems(Update, viscosity);
    }
}
