use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::consts::MOTOR_MAX_TARGET_V;

use super::{neuron::GenericBlockNN, signal::SignalHandler};

/// Bevy resource, which make sure the neurons can be accessed 
/// and modified from bevy side
/// 
/// index 0 is occupied by default,
/// which represent random neuron output.
#[derive(Resource,Debug)]
pub struct BevyBlockNeurons{
    pub nnvec:Vec<GenericBlockNN>
}

impl Default for BevyBlockNeurons {
    fn default() -> Self {
        let mut v = Vec::<GenericBlockNN>::new();
        v.push(GenericBlockNN { value: 0.0 });
        Self { 
            nnvec: v
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