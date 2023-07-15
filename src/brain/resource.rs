use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::consts::MOTOR_MAX_TARGET_V;

use super::{neuron::{BlockNN, GenericNN}, signal::SignalHandler};

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

    /// start neuron computing and return outputs
    pub fn get_outputs(&mut self, signal_handler: SignalHandler) -> Vec<[f32; 2]> {
        todo!()
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