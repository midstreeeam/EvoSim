use bevy::prelude::*;

use super::neuron::GenericBlockNN;

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