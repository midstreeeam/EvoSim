use ndarray::prelude::*;
use rand::prelude::*;

use crate::consts::*;

use super::signal::InwardNNInputSignal;

const CL: usize = INWARD_NN_CHILDREN_INPUT_LEN;

#[derive(Debug)]
pub enum GenericNN{
    BLOCKNN(BlockNN),
    BRAINNN(BrainNN)
}

/// neuron for blocks.
///
/// Each block should have two independent neurons:
/// InwardNN and OutwardNN
#[derive(Debug)]
pub struct BlockNN {
    pub value: f32,
}

impl Default for BlockNN {
    fn default() -> Self {
        todo!()
    }
}

impl BlockNN {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    /// output inward signal that passing to next layer
    /// Takes input layer's singal
    pub fn get_inward_output(&self, _:&InwardNNInputSignal) -> Array1<f32> {
        self.get_rand_inward_output()
    }

    pub fn get_rand_inward_output(&self) -> Array1<f32> {
        let mut rng = thread_rng();
        Array1::from_shape_fn((4,), |_| rng.gen::<f32>())
    }
}


/// NN for centeral brain
#[derive(Debug)]
pub struct BrainNN{
    pub value: f32
}

impl Default for BrainNN {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}