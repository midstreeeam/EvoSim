use bevy::prelude::*;
use bevy_rapier2d::{prelude::{ImpulseJoint, ContactForceEvent}, rapier::prelude::JointAxis};

use crate::{blob::block::NeuronId, consts::*, brain::resource::BevyBlockNeurons};

pub fn block_action(
    mut q:Query<(&Parent, &mut ImpulseJoint)>,
    nn_id_q: Query<&NeuronId>,
    bbn: ResMut<BevyBlockNeurons>,
    mut cf_events: EventReader<ContactForceEvent>
){
    for (parent, mut joint) in q.iter_mut(){
        let entity_id = parent.get();
        let NeuronId(nn_id) = nn_id_q.get(entity_id).unwrap_or(&NeuronId(0));

        let a = get_cf_event(entity_id, &mut cf_events);

        let signal = bbn.nnvec[*nn_id].get_rand_output();
        joint.data.set_motor_position(JointAxis::AngX, signal[0], MOTOR_STIFFNESS, MOTOR_DAMPING);
        joint.data.set_motor_velocity(JointAxis::AngX, signal[1], MOTOR_DAMPING);
    }
}


// TODO: test preformance and change to `get_bulk_cf_events()` if necessary
// loop over all entities over all events might be slow
// TODO: Takes input of mut ref, returns a clone, considering return the reference
// need to dealing with lifetime
//
/// get contact force event for an entity,
/// return the first if multiple event shappen at the same time
pub fn get_cf_event(
    entity_id: Entity,
    cf_events: &mut EventReader<ContactForceEvent>,
) -> Option<ContactForceEvent>{
    cf_events.iter().find(
        |&event| event.collider1==entity_id || event.collider2==entity_id
    ).and_then(|event| Some(event.clone()))
}
