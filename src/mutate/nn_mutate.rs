use rand::prelude::*;
use rand_distr::{Distribution, Normal};

use crate::{
    brain::{
        neuron::{BlockNN, BrainNN, GenericNN},
        nn::BaseNN,
    },
    consts::mutate_consts::*,
};

pub fn mutate_nn(nnvec: &mut Vec<GenericNN>) {
    for nn in nnvec.iter_mut() {
        let mut rng: ThreadRng = thread_rng();

        if !rng.gen_bool(MUTATE_NN_PORB as f64) {
            continue;
        }

        match nn {
            GenericNN::BRAINNN(nn) => mutate_brain_nn(nn),
            GenericNN::BLOCKNN(nn) => mutate_block_nn(nn),
        }
    }
}

fn mutate_block_nn(nn: &mut BlockNN) {
    mutate_base_nn(&mut nn.inward_nn.nn);
    mutate_base_nn(&mut nn.outward_nn.nn);
}

fn mutate_brain_nn(nn: &mut BrainNN) {
    mutate_base_nn(&mut nn.nn);
}

fn mutate_base_nn(nn: &mut BaseNN) {
    let normal = Normal::new(0.0, MUTATE_NN_STD).unwrap();

    // Use the thread_rng to get a thread-local random number generator
    let mut rng: ThreadRng = thread_rng();

    for layer in &mut nn.layers {
        // Mutate weights
        for weight in layer.weights.iter_mut() {
            *weight += normal.sample(&mut rng) as f32;
        }

        // Mutate biases
        for bias in layer.bias.iter_mut() {
            *bias += normal.sample(&mut rng) as f32;
        }
    }
}
