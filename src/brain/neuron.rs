use std::fmt;

use ndarray::prelude::*;
use rand::prelude::*;

use crate::consts::*;

use super::{signal::{InwardNNInputSignal, BrainSignal, OutwardNNInputSignal}, nn::BaseNN};

const CL: usize = INWARD_NN_CHILDREN_INPUT_LEN;

#[derive(Debug)]
pub enum GenericNN{
    BLOCKNN(BlockNN),
    BRAINNN(BrainNN)
}

#[derive(Debug)]
pub struct InwardNN{
    nn: BaseNN
}

impl Default for InwardNN {
    fn default() -> Self {
        Self {
            nn: BaseNN::new_rand(Vec::from_iter(INWARD_NN_SHAPE.into_iter().clone()), ACTIVATION_FUNCTION)
        }
    }
}

#[derive(Debug)]
pub struct OutwardNN{
    nn: BaseNN
}

impl Default for OutwardNN {
    fn default() -> Self {
        Self{
            nn: BaseNN::new_rand(Vec::from_iter(OUTWARD_NN_SHAPE.into_iter().clone()), ACTIVATION_FUNCTION)
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
    nn: BaseNN
}

impl Default for BrainNN {
    fn default() -> Self {
        Self {
            nn: BaseNN::new_rand(Vec::from_iter(BRAIN_NN_SHAPE.into_iter().clone()), ACTIVATION_FUNCTION)
        }
    }
}

impl BrainNN {
    pub fn forward(&self, signal: &BrainSignal) -> Array1<f32>{
        self.nn.forward(signal.to_array())
    }

    pub fn get_rand_brain_output(&self) -> Array1<f32> {
        let mut rng = thread_rng();
        Array1::from_shape_fn((4,), |_| rng.gen::<f32>())
    }
}

impl fmt::Display for BrainNN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.nn
        )
    }
}