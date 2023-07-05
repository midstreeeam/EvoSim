use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}


pub fn viscosity(
    mut block_q: Query<(&Collider, &Transform, &Velocity, &mut ExternalForce)>
){
    for (collider,transform,v,mut force) in block_q.iter_mut(){
        // skip objects not moving
        if v.linvel.length()==0.0{
            continue;
        }
        let cube_shape: Vec2 = collider.as_cuboid().unwrap().half_extents();
        let rotation: f32 = transform.rotation.to_axis_angle().1;

        let velocity_direction = v.linvel.normalize();
        let face_direction = Vec2::new(rotation.cos(), rotation.sin());
        let angle = velocity_direction.angle_between(face_direction).abs();
        let projected_area = (cube_shape.x * angle.sin().abs())
            + cube_shape.y * angle.cos().abs();

        // considering changing drag_coeff
        force.force = 5.0 * (-v.linvel * projected_area);

    }
}