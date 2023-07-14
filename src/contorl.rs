use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{ContactForceEvent, ImpulseJoint, Velocity},
    rapier::prelude::JointAxis,
};

use crate::{
    blob::block::{JointInfo, NeuronId},
    brain::resource::BevyBlockNeurons,
    componet::{BlobEntityIndex, ColliderFlag},
    consts::*,
};

// TODO: also select center block or add a new way to make center block's neuron work
// TODO: solve parallel problem, the first block receive output after the last block send signal
/// select `Query<(&Parent, &mut ImpulseJoint)` 
/// means the center block will not be selected
/// 
/// Can not use `EventReader` multiple times each frame.
/// Events been read will be marked as read.
pub fn block_action(
    mut q: Query<(&Parent, &mut ImpulseJoint)>,
    nn_id_q: Query<&NeuronId>,
    bbn: ResMut<BevyBlockNeurons>,
    mut cf_events: EventReader<ContactForceEvent>,
    collider_q: Query<&ColliderFlag>,
) {

    let mut cf_events_vec = Vec::from_iter(cf_events.into_iter().cloned());

    for (parent, mut joint) in q.iter_mut() {
        let entity_id = parent.get();
        let NeuronId {
            id: nn_id,
            parent_id: _,
        } = nn_id_q.get(entity_id).unwrap_or(&NeuronId {
            id: 0,
            parent_id: None,
        });

        // if contact
        if let Some(_) = get_cf_signal(entity_id, &mut cf_events_vec, &collider_q) {}

        let signal = bbn.nnvec[*nn_id].get_rand_output();
        joint
            .data
            .set_motor_position(JointAxis::AngX, signal[0], MOTOR_STIFFNESS, MOTOR_DAMPING);
        joint
            .data
            .set_motor_velocity(JointAxis::AngX, signal[1], MOTOR_DAMPING);
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
/// Output singal depends on NN's input
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

        if let (
            Ok(ColliderFlag::BLOCK(BlobEntityIndex(Some(sid)))),
            Ok(oflag)) =
            (blob_flag_q.get(entity_id), blob_flag_q.get(other)
        ) {
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
pub fn update_joint_info(
    parent_joint_q: Query<(&Parent, &ImpulseJoint)>,
    mut joint_info_q: Query<&mut JointInfo>,
    trans_q: Query<&Transform>,
    veloc_q: Query<&Velocity>,
) {
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
}

fn get_relative_rotation(transform1: &Transform, transform2: &Transform) -> f32 {
    let r1 = transform1.rotation;
    let r2 = transform2.rotation;
    r1.z.atan2(r1.w) * 360.0 / PI - r2.z.atan2(r2.w) * 360.0 / PI
}

fn get_relative_angular_velocity(v1: &Velocity, v2: &Velocity) -> f32 {
    (v1.angvel - v2.angvel) / PI * 180.0
}
