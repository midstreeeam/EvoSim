use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{Collider, ContactForceEvent, ImpulseJoint, Velocity},
    rapier::prelude::JointAxis,
};

use crate::{
    blob::{
        blob::BlobInfo,
        block::{BlockDepth, CenterBlockFlag, JointInfo, NeuronId, ParentAnchor},
    },
    brain::{
        resource::BevyBlockNeurons,
        signal::{BrainSignal, InwardNNInputSignal, SignalHandler},
    },
    componet::{BlobEntityIndex, ColliderFlag},
    consts::*,
};

/// select `Query<(&Parent, &mut ImpulseJoint)`
/// means the center block will not be selected
///
/// Can not use `EventReader` multiple times each frame.
/// Events been read will be marked as read.
pub fn block_action(
    mut brain_q: Query<(&Parent, Entity), With<CenterBlockFlag>>,
    mut block_q: Query<(&Parent, &mut ImpulseJoint)>,
    nn_id_q: Query<&NeuronId>,
    mut bbn: ResMut<BevyBlockNeurons>,
    mut cf_events: EventReader<ContactForceEvent>,
    collider_q: Query<&ColliderFlag>,
    joint_info_q: Query<&JointInfo>,
    depth_q: Query<&BlockDepth>,
    blob_q: Query<&BlobInfo>,
    p_anchor_q: Query<&ParentAnchor>
) {
    let mut signal_handler = SignalHandler::default();

    let mut cf_events_vec = Vec::from_iter(cf_events.into_iter().cloned());

    // push inward
    for (parent, joint) in block_q.iter_mut() {
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
            entity_id.index()
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
    let output = bbn.get_rand_outputs(signal_handler);

    // TODO: make sure the element order in output vec matches the iterator so that they can be zipped together
    // update physical world
    for (signal, (_, mut joint)) in output.iter().zip(block_q.iter_mut()) {
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

pub fn update_blob_info(
    tc_q: Query<(&Transform, &Collider)>,
    mut blob_q: Query<(&mut BlobInfo, &Children)>,
) {
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
        blob.mass_center = new_mass_center;
    }
}

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
