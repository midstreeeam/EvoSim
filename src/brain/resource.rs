use std::f32::consts::PI;

use bevy::prelude::*;
use ndarray::prelude::*;
use rand::prelude::*;

use crate::{
    brain::signal::InwardNNInputSignalUnit,
    consts::{MOTOR_MAX_TARGET_V, OUTWARD_NN_PARENT_INPUT_LEN},
};

use super::{
    neuron::GenericNN,
    signal::{BrainSignalUnit, SignalHandler},
};

const DL: usize = OUTWARD_NN_PARENT_INPUT_LEN;

// TODO: add random generator and oscillator
/// Bevy resource, which make sure the neurons can be accessed
/// and modified from bevy side
#[derive(Resource, Debug)]
pub struct BevyBlockNeurons {
    pub nnvec: Vec<GenericNN>,
}

impl Default for BevyBlockNeurons {
    fn default() -> Self {
        let nnv = Vec::<GenericNN>::new();
        Self { nnvec: nnv }
    }
}

impl BevyBlockNeurons {
    // TODO: parallel, gpu
    /// start neuron computing and return outputs
    pub fn get_outputs(&mut self, mut signal_handler: SignalHandler) -> Vec<(Entity, f32, f32)> {
        // store output value for joint motors
        let mut outputs: Vec<(Entity, f32, f32)> = Vec::new();
        // store internal outward_nn's outputs, index is nn_id
        let mut outward_passes = vec![Array1::<f32>::zeros(DL); self.nnvec.len()];

        // generate grouped signal
        let (mut grouped_signal, mut brain_signal) = signal_handler.get_sig_mut();

        // println!("grouped signal {:#?}",grouped_signal);
        // println!("brain signal {:#?}",brain_signal);
        // passing through all inward layers
        for idx in (1..grouped_signal.len()).rev() {
            inward_bulk_pass(&mut grouped_signal, &mut self.nnvec, idx)
        }

        // passing to brain
        brain_pass(&mut brain_signal, &grouped_signal[0], &mut self.nnvec);
        // println!("{:#?}",brain_signal[0].signal);
        brain_forward(&brain_signal, &mut self.nnvec, &mut outward_passes);

        for idx in 0..grouped_signal.len() {
            outward_bulk_pass(
                &mut grouped_signal,
                &mut self.nnvec,
                idx,
                &mut outputs,
                &mut outward_passes,
            )
        }

        outputs
    }

    pub fn get_rand_outputs(&self, signal_handler: SignalHandler) -> Vec<[f32; 2]> {
        let mut rng = thread_rng();
        let len = signal_handler.inward_len();
        vec![
            [
                rng.gen_range(-PI..PI),
                rng.gen_range(-MOTOR_MAX_TARGET_V..MOTOR_MAX_TARGET_V),
            ];
            len
        ]
    }

    /// drop all the values inside
    pub fn clear(&mut self) {
        self.nnvec.clear();
    }
}

/// Pass the signal from the leaf to the root layer by layer
///
/// bulk_idx can not be 0
fn inward_bulk_pass(
    grouped_signal: &mut Vec<Vec<&mut InwardNNInputSignalUnit>>,
    nnvec: &mut Vec<GenericNN>,
    bulk_idx: usize,
) {
    if bulk_idx == 0 {
        panic!()
    }

    // aviod multiple borrow here
    let (left, right) = grouped_signal.split_at_mut(bulk_idx);
    let passed_layer: &mut Vec<&mut InwardNNInputSignalUnit> = &mut left[bulk_idx - 1];
    let current_layer: &mut Vec<&mut InwardNNInputSignalUnit> = &mut right[0];

    // TODO: parallel this for loop
    for unit in current_layer {
        if let GenericNN::BLOCKNN(nn) = &mut nnvec[unit.nn_id] {
            passed_layer
                .iter_mut()
                .find(|u| u.nn_id == unit.parent_nn_id)
                .unwrap()
                .get_signal_mut()
                .push_child_signal(nn.get_inward_output(&unit.signal), unit.anchor_pos);
        } else {
            panic!(
                "nn with id {} is expected to be BLOCKNN, but found BRAINNN",
                unit.nn_id
            )
        }
    }
}

/// pass the signal from last inward layer to brain
fn brain_pass(
    brain_signal: &mut Vec<&mut BrainSignalUnit>,
    current_layer: &Vec<&mut InwardNNInputSignalUnit>,
    nnvec: &mut Vec<GenericNN>,
) {
    for unit in current_layer {
        if let GenericNN::BLOCKNN(nn) = &mut nnvec[unit.nn_id] {
            brain_signal
                .iter_mut()
                .find(|u: &&mut &mut BrainSignalUnit| u.nn_id == unit.parent_nn_id)
                .unwrap()
                .get_signal_mut()
                .push_child_signal(nn.get_inward_output(&unit.signal), unit.anchor_pos);
        } else {
            panic!(
                "nn with id {} is expected to be BLOCKNN, but found BRAINNN",
                unit.nn_id
            )
        }
    }
}

/// run brain_nn and start outward pass
fn brain_forward(
    brain_signal: &Vec<&mut BrainSignalUnit>,
    nnvec: &mut Vec<GenericNN>,
    outward_passes: &mut Vec<Array1<f32>>,
) {
    for signal in brain_signal {
        if let Some(GenericNN::BRAINNN(brain)) = nnvec.get(signal.nn_id) {
            // println!("{:#?}",signal.signal);
            // store forward result
            outward_passes[signal.nn_id] = brain.forward(&signal.signal);
        } else {
            panic!()
        }
    }
}

fn outward_bulk_pass(
    grouped_signal: &mut Vec<Vec<&mut InwardNNInputSignalUnit>>,
    nnvec: &mut Vec<GenericNN>,
    bulk_idx: usize,
    outputs: &mut Vec<(Entity, f32, f32)>,
    outward_passes: &mut Vec<Array1<f32>>,
) {
    let current_layer = &grouped_signal[bulk_idx];

    // TODO: parallel this loop
    for unit in current_layer {
        if let GenericNN::BLOCKNN(nn) = &mut nnvec[unit.nn_id] {
            // get result from parent and write output back
            let a = nn.get_outward_output(&outward_passes[unit.parent_nn_id]);
            outward_passes[unit.nn_id] = a.slice(s![..DL]).map(|x| *x).clone();
            // push result
            outputs.push((unit.entity_id, a[DL], a[DL + 1]));
        } else {
            panic!()
        }
    }
}
