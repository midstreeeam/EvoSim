//! all implementation of framely updates relate to blobs (sensors and activators behavior)

use std::f32::consts::PI;
use std::time::Instant;

use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{Collider, ContactForceEvent, ImpulseJoint, Velocity},
    rapier::prelude::JointAxis,
};

use crate::{
    blob::{
        blob::BlobInfo,
        block::{BlockDepth, CenterBlockFlag, JointInfo, NeuronId, ParentAnchor},
        geno_blob_builder::BlobGeno,
    },
    brain::{
        resource::BevyBlockNeurons,
        signal::{BrainSignal, InwardNNInputSignal, SignalHandler},
    },
    componet::{BlobEntityIndex, ColliderFlag},
    consts::*,
};

use super::resource::{Frames, TED};

/// **CORE FUNCTION**
///
/// Update all blobs' motor condition to let blobs preform action.
///
/// select `Query<(&Parent, &mut ImpulseJoint)`
/// means the center block will not be selected
///
/// Can not use `EventReader` multiple times each frame.
/// Events been read will be marked as read.
pub fn block_action(
    mut brain_q: Query<(&Parent, Entity), With<CenterBlockFlag>>,
    mut block_q: Query<(Entity, &Parent, &mut ImpulseJoint)>,
    nn_id_q: Query<&NeuronId>,
    mut bbn: ResMut<BevyBlockNeurons>,
    mut cf_events: EventReader<ContactForceEvent>,
    collider_q: Query<&ColliderFlag>,
    joint_info_q: Query<&JointInfo>,
    depth_q: Query<&BlockDepth>,
    blob_q: Query<&BlobInfo>,
    p_anchor_q: Query<&ParentAnchor>,
    // mut joint_q: Query<&mut ImpulseJoint>
) {
    let start_time = Instant::now();

    if block_q.is_empty() {
        assert!(brain_q.is_empty());
        return;
    }

    if brain_q.is_empty() {
        assert!(block_q.is_empty());
        return;
    }

    let mut signal_handler = SignalHandler::default();
    let mut cf_events_vec = Vec::from_iter(cf_events.into_iter().cloned());

    // push inward
    for (child, parent, joint) in block_q.iter_mut() {
        let entity_id = parent.get();

        // get id
        let NeuronId {
            id: nn_id,
            parent_id: parent_nn_id,
        } = nn_id_q.get(entity_id).unwrap_or(&NeuronId {
            id: 0,
            parent_id: None,
        });

        // init signal
        let cf_singal = get_cf_signal(entity_id, &mut cf_events_vec, &collider_q);
        let joint_motor = joint.data.motor(JointAxis::AngX).unwrap();
        let joint_info = joint_info_q.get(entity_id).unwrap();
        let joint_signal = (
            joint_motor.target_pos,
            joint_motor.target_vel,
            joint_info.ang_pos,
            joint_info.ang_velocity,
        );
        let inward_signal = InwardNNInputSignal::default()
            .with_cf_signal(cf_singal)
            .with_joint_singal(joint_signal);

        // push inward signals to signal handler
        // unwarp parent_id, since all inward signal should have parent
        // unwarp depth, since all inward signal should have depth
        // unwrap p_anchor, since all inward signal should have parent_anchor
        signal_handler.push_inward(
            inward_signal,
            *nn_id,
            parent_nn_id.unwrap(),
            depth_q.get(entity_id).unwrap(),
            p_anchor_q.get(entity_id).unwrap(),
            child,
        );
    }

    // push brains
    for (parent, entity_id) in brain_q.iter_mut() {
        // get id
        // should have id so unwrap
        let nn_id = nn_id_q.get(entity_id).unwrap().id;
        // cf_signal
        let cf_signal = get_cf_signal(entity_id, &mut cf_events_vec, &collider_q);
        // blob_signal
        // should in blobinfo so unwrap
        let blobinfo = blob_q.get(parent.get()).unwrap();

        signal_handler.push_brain(
            BrainSignal::default()
                .with_cf_signal(cf_signal)
                .with_blob_info(blobinfo.mass_center, blobinfo.velocity),
            nn_id,
        );
    }

    // run neuron
    let output: Vec<(Entity, f32, f32)> = bbn.get_outputs(signal_handler);

    // println!("{}",output[1].1);
    // update joints base on nn's output
    for (entity_id, target_pos, target_vel) in output {
        // println!("{},{}",target_pos,target_vel);
        let (_, _, mut joint) = block_q.get_mut(entity_id).unwrap();
        joint
            .data
            .set_motor_position(JointAxis::AngX, target_pos, MOTOR_STIFFNESS, MOTOR_DAMPING);
        joint
            .data
            .set_motor_velocity(JointAxis::AngX, target_vel, MOTOR_DAMPING);
    }

    // let output = bbn.get_rand_outputs(signal_handler);
    // // TODO: make sure the element order in output vec matches the iterator so that they can be zipped together
    // // update physical world
    // for (signal, (_, mut joint)) in output.iter().zip(block_q.iter_mut()) {
    //     joint
    //         .data
    //         .set_motor_position(JointAxis::AngX, signal[0], MOTOR_STIFFNESS, MOTOR_DAMPING);
    //     joint
    //         .data
    //         .set_motor_velocity(JointAxis::AngX, signal[1], MOTOR_DAMPING);
    // }
    let duration = Instant::now() - start_time;
    if PRINT_FUNCTION_TIME && duration >= MIN_PRINT_DURATION {
        println!("block_action: {:?}", duration);
    }
}

// TODO: test preformance and change to `get_bulk_cf_events()` if necessary
// loop over all entities over all events might be slow
// TODO: Takes input of mut ref, returns a clone, considering return the reference
// need to dealing with lifetime
//
/// get contact force event for an entity,
/// return the first if multiple event shappen at the same time
fn get_cf_event(
    entity_id: Entity,
    cf_events: &mut Vec<ContactForceEvent>,
) -> Option<ContactForceEvent> {
    cf_events
        .iter()
        .find(|&event| event.collider1 == entity_id || event.collider2 == entity_id)
        .and_then(|event| Some(event.clone()))
}

/// Not a bevy system.
///
/// Output singal depends on NN's input
///
/// collect contact force events and translate it into nn signals
///
/// contact blob and contact wall have different signal
fn get_cf_signal(
    entity_id: Entity,
    cf_events_vec: &mut Vec<ContactForceEvent>,
    blob_flag_q: &Query<&ColliderFlag>,
) -> Option<(bool, bool, [f32; 2], f32)> {
    // if contact
    if let Some(event) = get_cf_event(entity_id, cf_events_vec) {
        let other = if entity_id == event.collider1 {
            event.collider2
        } else {
            event.collider1
        };

        if let (Ok(ColliderFlag::BLOCK(BlobEntityIndex(Some(sid)))), Ok(oflag)) =
            (blob_flag_q.get(entity_id), blob_flag_q.get(other))
        {
            let (mut wall, mut blob, vect, mag) =
                (false, false, event.total_force, event.total_force_magnitude);
            if let ColliderFlag::WALL = oflag {
                wall = true;
            }
            if let ColliderFlag::BLOCK(BlobEntityIndex(Some(oid))) = oflag {
                if sid != oid {
                    blob = true;
                }
            }
            return Some((wall, blob, [vect.x, vect.y], mag));
        }
    }
    None
}

/// Update `JointInfo` componet each frame.
///
/// update:
/// - current angular velocity
/// - current angular position
pub fn update_joint_info(
    parent_joint_q: Query<(&Parent, &ImpulseJoint)>,
    mut joint_info_q: Query<&mut JointInfo>,
    trans_q: Query<&Transform>,
    veloc_q: Query<&Velocity>,
) {
    let start_time = Instant::now();
    for (parent, joint) in parent_joint_q.iter() {
        let parent_id = parent.get();
        let child_id = joint.parent;

        // get info
        let joint_info = joint_info_q.get_mut(parent_id);
        let parent_trans = trans_q.get(parent_id);
        let child_trans = trans_q.get(child_id);
        let parent_v = veloc_q.get(parent_id);
        let chlid_v = veloc_q.get(child_id);

        // update info
        if let (Ok(mut ji), Ok(pt), Ok(ct), Ok(pv), Ok(cv)) =
            (joint_info, parent_trans, child_trans, parent_v, chlid_v)
        {
            ji.update(
                get_relative_rotation(pt, ct),
                get_relative_angular_velocity(pv, cv),
            );
        } else {
            panic!("update joint info failed!")
        }
    }
    let duration = Instant::now() - start_time;
    if PRINT_FUNCTION_TIME && duration >= MIN_PRINT_DURATION {
        println!("update_joint_info: {:?}", duration);
    }
}

/// Calculates the relative rotation between two transforms.
///
/// This function takes two references to `Transform` objects and computes the relative rotation
/// between them in degrees. The rotation is calculated using the arctangent of the z and w components
/// of the rotation quaternion.
///
/// # Parameters
///
/// * `transform1`: A reference to the first transform.
/// * `transform2`: A reference to the second transform.
///
/// # Returns
///
/// Returns the relative rotation between the two transforms in degrees.
pub fn get_relative_rotation(transform1: &Transform, transform2: &Transform) -> f32 {
    let r1 = transform1.rotation;
    let r2 = transform2.rotation;
    r1.z.atan2(r1.w) * 360.0 / PI - r2.z.atan2(r2.w) * 360.0 / PI
}

/// Calculates the relative angular velocity between two velocity objects.
///
/// This function takes two references to `Velocity` objects and computes the relative angular
/// velocity between them in degrees per second. The result is normalized by dividing by π and
/// multiplying by 180.
///
/// # Parameters
///
/// * `v1`: A reference to the first velocity object.
/// * `v2`: A reference to the second velocity object.
///
/// # Returns
///
/// Returns the relative angular velocity between the two velocity objects in degrees per second.
pub fn get_relative_angular_velocity(v1: &Velocity, v2: &Velocity) -> f32 {
    (v1.angvel - v2.angvel) / PI * 180.0
}

/// **a bevy function**
///
/// Updates the `BlobInfo` for every blob component in the ECS.
///
/// This function iterates through all blob components and calculates the new mass center and velocity
/// based on the child entities' transforms and colliders. It also updates the move distance of the blob
/// based on the current frame and iteration length.
/// 
/// it updates:
/// - mass_center
/// - velocity
/// - cumulated move distance (for move training usage)
/// 
/// # Panics
///
/// This function will panic if any child of a blob does not have both a transform and collider, 
/// or if a blob does not have at least one block.
pub fn update_blob_info(
    tc_q: Query<(&Transform, &Collider)>,
    mut blob_q: Query<(&mut BlobInfo, &Children)>,
    frames: Res<Frames>,
) {
    let start_time = Instant::now();
    for (mut blob, children) in blob_q.iter_mut() {
        let mut mass_vec = Vec::<[f32; 3]>::new();
        for child in children {
            // unwrap since every child of blob should have transform and collider
            let (transform, collider) = tc_q.get(*child).unwrap();
            mass_vec.push([
                transform.translation.x,
                transform.translation.y,
                collider.scale().x * collider.scale().y,
            ])
        }
        // unwrap since all blob should have at least one block
        let new_mass_center = get_mass_center(mass_vec).unwrap();
        blob.velocity = [
            new_mass_center[0] - blob.mass_center[0],
            new_mass_center[1] - blob.mass_center[1],
        ];

        // update move_distance
        if frames.0 % ITERATION_LENGTH as u128 != 1 {
            blob.move_distance[0] += blob.velocity[0];
            blob.move_distance[1] += blob.velocity[1];
        }

        // update mass_center
        blob.mass_center = new_mass_center;
    }
    let duration = Instant::now() - start_time;
    if PRINT_FUNCTION_TIME && duration >= MIN_PRINT_DURATION {
        println!("update_blob_info: {:?}", duration);
    }
}

/// Calculates the mass center of a collection of points.
///
/// This function takes a vector of points, where each point is represented as an array of three `f32` values.
/// The first two values are the x and y coordinates, and the third value is the mass at that point.
/// The function calculates the weighted sum of the coordinates based on the mass and divides by the total mass
/// to find the mass center.
///
/// # Parameters
///
/// * `mass_points`: A vector of points, where each point is an array `[x, y, mass]`.
///
/// # Returns
///
/// Returns an `Option` containing an array `[x, y]` representing the mass center of the points.
/// If the input vector is empty or the total mass is zero, the function returns `None`.
fn get_mass_center(mass_points: Vec<[f32; 3]>) -> Option<[f32; 2]> {
    if mass_points.is_empty() {
        return None;
    }

    let mut total_mass = 0.0;
    let mut weighted_sum = [0.0, 0.0];

    for point in mass_points {
        let mass = point[2];
        weighted_sum[0] += point[0] * mass;
        weighted_sum[1] += point[1] * mass;
        total_mass += mass;
    }

    if total_mass == 0.0 {
        None
    } else {
        Some([weighted_sum[0] / total_mass, weighted_sum[1] / total_mass])
    }
}

/// update iteration resource
pub fn update_iteration_frames(mut frames: ResMut<Frames>) {
    frames.0 += 1;
}

/// update TED resource
pub fn update_crowding_distance(
    mut blob_q: Query<(&BlobGeno, &mut BlobInfo)>,
    mut ted: ResMut<TED>,
) {
    let mut genovec: Vec<&BlobGeno> = Vec::new();
    let mut infovec: Vec<BlobInfo> = Vec::new();
    for (geno, info) in blob_q.iter() {
        genovec.push(geno);
        infovec.push(info.clone());
    }

    // calculate crowding distance
    for i in 0..genovec.len() {
        let &this_geno = genovec.get(i).unwrap();
        let this_info = infovec.get_mut(i).unwrap();
        let mut sum_crowding_distance: usize = 0;
        for j in 0..genovec.len() {
            let &other_geno = genovec.get(j).unwrap();
            sum_crowding_distance +=
                this_geno.vec_tree.tree_edit_distance(&other_geno.vec_tree) as usize;
        }
        let avg_corwding_distance = sum_crowding_distance as f32 / genovec.len() as f32;
        this_info.crowding_distance = avg_corwding_distance;
    }

    let mut cd: f32 = 0.0;
    // update crowding distance
    for (i, (_, mut info)) in blob_q.iter_mut().enumerate() {
        *info = infovec.get(i).unwrap().clone();
        cd += infovec.get(i).unwrap().crowding_distance;
    }

    ted.0 = cd / blob_q.iter().len() as f32;
}
