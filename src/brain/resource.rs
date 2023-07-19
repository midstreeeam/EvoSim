use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{consts::MOTOR_MAX_TARGET_V, brain::signal::InwardNNInputSignalUnit};

use super::{neuron::GenericNN, signal::{SignalHandler, BrainSignalUnit}};

/// Bevy resource, which make sure the neurons can be accessed 
/// and modified from bevy side
#[derive(Resource,Debug)]
pub struct BevyBlockNeurons{
    pub nnvec:Vec<GenericNN>,
}

impl Default for BevyBlockNeurons {
    fn default() -> Self {
        let nnv = Vec::<GenericNN>::new();
        Self { 
            nnvec: nnv,
        }
    }
}

impl BevyBlockNeurons {

    // TODO: parallel, gpu
    /// start neuron computing and return outputs
    pub fn get_outputs(&self, mut signal_handler: SignalHandler) -> Vec<[f32; 2]> {

        // generate grouped signal
        let (mut grouped_signal,mut brain_signal) = signal_handler.get_sig_mut();
        
        // println!("{:#?}",grouped_signal);
        // println!("{:#?}",brain_signal);
        // passing through all inward layers
        for idx in (1..grouped_signal.len()).rev(){
            // println!("{}",idx);
            inward_bulk_pass(&mut grouped_signal, &self.nnvec, idx)
        }

        // passing to brain
        brain_pass(&mut brain_signal, &grouped_signal[0], &self.nnvec);
        // println!("{:#?}",brain_signal[0].signal);

        // return random yet
        self.get_rand_outputs(signal_handler)
    }

    pub fn get_rand_outputs(&self, signal_handler: SignalHandler) -> Vec<[f32; 2]> {
        let mut rng = thread_rng();
        let len = signal_handler.len();
        vec![[
                rng.gen_range(-PI..PI),
                rng.gen_range(-MOTOR_MAX_TARGET_V..MOTOR_MAX_TARGET_V),
            ]; len
        ]
    }
}

/// Pass the signal from the leaf to the root layer by layer
/// 
/// bulk_idx can not be 0
fn inward_bulk_pass(
    grouped_signal: &mut Vec<Vec<&mut InwardNNInputSignalUnit>>,
    nnvec: &Vec<GenericNN>, bulk_idx:usize
){
    if bulk_idx == 0{
        panic!()
    }

    // aviod multiple borrow here
    let (left, right) = grouped_signal.split_at_mut(bulk_idx);
    let passed_layer = &mut left[bulk_idx - 1];
    let current_layer = &mut right[0];

    // TODO: parallel this for loop
    for unit in current_layer {
        if let GenericNN::BLOCKNN(nn) = &nnvec[unit.nn_id] {
            passed_layer.iter_mut()
                .find(|u| u.nn_id == unit.parent_nn_id)
                .unwrap()
                .get_signal_mut()
                .push_child_signal(nn.get_inward_output(&unit.signal), unit.anchor_pos);
        } else {
            panic!()
        }
    }

}

/// pass the signal from last inward layer to brain
fn brain_pass(
    brain_signal: &mut Vec<&mut BrainSignalUnit>,
    current_layer: &Vec<&mut InwardNNInputSignalUnit>,
    nnvec: &Vec<GenericNN>
){
    for unit in current_layer{
        if let GenericNN::BLOCKNN(nn) = &nnvec[unit.nn_id] {
            brain_signal.iter_mut()
                .find(|u| u.nn_id == unit.parent_nn_id)
                .unwrap()
                .get_signal_mut()
                .push_child_signal(nn.get_inward_output(&unit.signal), unit.anchor_pos);
        } else {
            panic!()
        }
    }
}
