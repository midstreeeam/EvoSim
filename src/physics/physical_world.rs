//! implementation of `PhysiWorldPlugin`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::physics::rules::*;
use crate::physics::world::setup_walls;

/// all implementations relate to physic and the world.
/// 
/// includes:
/// - bevy plugin
/// - world setup
/// - gravity setup
/// - viscosity force
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
        .add_systems(Update, viscosity)
        .add_plugins((
            // raiper
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ));
    }
}
