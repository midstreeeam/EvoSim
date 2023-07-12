use bevy::prelude::*;

use super::neuron::BlockNeuron;

#[derive(Resource)]
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