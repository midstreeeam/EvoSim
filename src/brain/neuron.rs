use ndarray::prelude::*;

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

    /// output the motor target pos and motor target v
    /// Takes input layer's singal
    pub fn get_output(&self, signal:&InwardNNInputSignal) -> Array1<f32> {

        todo!()
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