use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

/* Apply forces and impulses inside of a system. */
pub fn apply_forces(
    mut ext_forces: Query<&mut ExternalForce>,
    mut ext_impulses: Query<&mut ExternalImpulse>
) {
    // Apply forces.
    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = Vec2::new(-500.0, -500.0);
        ext_force.torque = 0.4;
    }

    // Apply impulses.
    for mut ext_impulse in ext_impulses.iter_mut() {
        ext_impulse.impulse = Vec2::new(100.0, 200.0);
        ext_impulse.torque_impulse = 0.4;
    }
}