use bevy::prelude::*;
use bevy_rapier2d::{prelude::ImpulseJoint, rapier::prelude::JointAxis};

use crate::{blob::block::NeuronId, consts::*, brain::resource::BevyBlockNeurons};

pub fn block_action(
    mut q:Query<(&Parent, &mut ImpulseJoint)>,
    nn_id_q: Query<&NeuronId>,
    bbn: ResMut<BevyBlockNeurons>
){
    for (parent, mut joint) in q.iter_mut(){
        let NeuronId(nn_id) = nn_id_q.get(parent.get()).unwrap_or(&NeuronId(0));
        // println!("{:#?} and {:#?}",parent.get(), nn_id);

        let signal = bbn.nnvec[*nn_id].get_rand_output();
        joint.data.set_motor_position(JointAxis::AngX, signal[0], MOTOR_STIFFNESS, MOTOR_DAMPING);
        joint.data.set_motor_velocity(JointAxis::AngX, signal[1], MOTOR_DAMPING);
    }
}