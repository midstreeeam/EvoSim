use bevy::prelude::*;

use super::neuron::BlockNeuron;

/// index 0 is occupied by default,
/// which represent random neuron output.
#[derive(Resource,Debug)]
pub struct BevyBlockNeurons{
    pub nnvec:Vec<BlockNeuron>
}

impl Default for BevyBlockNeurons {
    fn default() -> Self {
        let mut v = Vec::<BlockNeuron>::new();
        v.push(BlockNeuron { value: 0.0 });
        Self { 
            nnvec: v
        }
    }
}