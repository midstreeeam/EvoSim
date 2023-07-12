use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::consts::*;

pub fn setup_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}


/// Create drag force for under-water simulation
/// Cost about 5% of total running time in Physical Simulation
pub fn viscosity(
    mut block_q: Query<(&Collider, &Transform, &Velocity, &mut ExternalForce)>
){

    // parallel implementation, save about 3% of running time (in physical simulation)
    block_q
        .par_iter_mut()
        .for_each_mut(|(collider,transform,v,mut force)|{
            // skip objects not moving
            if v.linvel.length().abs() < EPSILON{
                return;
            }
            let cube_shape: Vec2 = collider.as_cuboid().unwrap().half_extents();
            let rotation: f32 = transform.rotation.to_axis_angle().1;

            let velocity_direction = v.linvel.normalize();
            let face_direction = Vec2::new(rotation.cos(), rotation.sin());
            let angle = velocity_direction.angle_between(face_direction).abs();
            let projected_area = (cube_shape.x * angle.sin().abs())
                + cube_shape.y * angle.cos().abs();

            // considering changing drag_coeff
            force.force = DRAG_COEFF * (-v.linvel * projected_area);
        });


        // sequencial implementation

    // for (collider,transform,v,mut force) in block_q.iter_mut(){
    //     // skip objects not moving
    //     if v.linvel.length().abs() < EPSILON{
    //         continue;
    //     }
    //     let cube_shape: Vec2 = collider.as_cuboid().unwrap().half_extents();
    //     let rotation: f32 = transform.rotation.to_axis_angle().1;

    //     let velocity_direction = v.linvel.normalize();
    //     let face_direction = Vec2::new(rotation.cos(), rotation.sin());
    //     let angle = velocity_direction.angle_between(face_direction).abs();
    //     let projected_area = (cube_shape.x * angle.sin().abs())
    //         + cube_shape.y * angle.cos().abs();

    //     // considering changing drag_coeff
    //     force.force = DRAG_COEFF * (-v.linvel * projected_area);
    // }

}