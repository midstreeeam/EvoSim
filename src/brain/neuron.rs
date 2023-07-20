use ndarray::prelude::*;
use rand::prelude::*;

use crate::consts::*;

use super::signal::{InwardNNInputSignal, BrainSignal, OutwardNNInputSignal};

const CL: usize = INWARD_NN_CHILDREN_INPUT_LEN;

#[derive(Debug)]
pub enum GenericNN{
    BLOCKNN(BlockNN),
    BRAINNN(BrainNN)
}

#[derive(Debug)]
pub struct InwardNN{

}

impl Default for InwardNN {
    fn default() -> Self {
        Self {  }
    }
}

#[derive(Debug)]
pub struct OutwardNN{

}

impl Default for OutwardNN {
    fn default() -> Self {
        Self{

        }
    }
}

/// neuron for blocks.
///
/// Each block should have two independent neurons:
/// InwardNN and OutwardNN
#[derive(Debug)]
pub struct BlockNN {
    pub inward_nn: InwardNN,
    pub outward_nn: OutwardNN,
    pub outward_signal: OutwardNNInputSignal
}

impl Default for BlockNN {
    fn default() -> Self {
        Self {
            inward_nn: InwardNN::default(),
            outward_nn: OutwardNN::default(),
            outward_signal: OutwardNNInputSignal::default()
        }
    }
}

impl BlockNN {
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

impl BrainNN {
    pub fn get_brain_output(signal: BrainSignal) {

    }

    pub fn get_rand_brain_output(&self) -> Array1<f32> {
        let mut rng = thread_rng();
        Array1::from_shape_fn((4,), |_| rng.gen::<f32>())
    }
}