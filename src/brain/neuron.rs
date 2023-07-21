use std::fmt;

use ndarray::prelude::*;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::consts::*;

use super::{
    nn::BaseNN,
    signal::{BrainSignal, InwardNNInputSignal, OutwardNNInputSignal},
};

const CL: usize = INWARD_NN_CHILDREN_INPUT_LEN;
const DL: usize = OUTWARD_NN_PARENT_INPUT_LEN;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenericNN {
    BLOCKNN(BlockNN),
    BRAINNN(BrainNN),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InwardNN {
    nn: BaseNN,
}

impl Default for InwardNN {
    fn default() -> Self {
        Self {
            nn: BaseNN::new_rand(
                Vec::from_iter(INWARD_NN_SHAPE.into_iter().clone()),
                ACTIVATION_FUNCTION,
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutwardNN {
    nn: BaseNN,
}

impl Default for OutwardNN {
    fn default() -> Self {
        Self {
            nn: BaseNN::new_rand(
                Vec::from_iter(OUTWARD_NN_SHAPE.into_iter().clone()),
                ACTIVATION_FUNCTION,
            ),
        }
    }
}

/// neuron for blocks.
///
/// Each block should have two independent neurons:
/// InwardNN and OutwardNN
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockNN {
    pub inward_nn: InwardNN,
    pub outward_nn: OutwardNN,
    pub outward_signal: OutwardNNInputSignal,
}

impl Default for BlockNN {
    fn default() -> Self {
        Self {
            inward_nn: InwardNN::default(),
            outward_nn: OutwardNN::default(),
            outward_signal: OutwardNNInputSignal::default(),
        }
    }
}

impl BlockNN {
    /// forward function for inward nn
    ///
    /// also update the `inherited` element in outward nn
    fn inward_forward(&mut self, signal: &InwardNNInputSignal) -> Array1<f32> {
        let array_signal = signal.to_array();
        // save duplicate signals for ourward usage
        self.outward_signal.inherit(&array_signal);
        self.inward_nn.nn.forward(array_signal.clone())
    }

    /// output inward signal that passing to next layer
    /// Takes input layer's singal
    pub fn get_inward_output(&mut self, signal: &InwardNNInputSignal) -> Array1<f32> {
        self.inward_forward(signal)
    }

    pub fn get_rand_inward_output(&self) -> Array1<f32> {
        let mut rng = thread_rng();
        Array1::from_shape_fn((4,), |_| rng.gen::<f32>())
    }

    pub fn get_outward_output(&mut self, parent_signal: &Array1<f32>) -> Array1<f32> {
        assert_eq!(parent_signal.len(), DL);
        self.outward_signal.parent_input = parent_signal.clone();
        self.outward_nn.nn.forward(self.outward_signal.to_array())
    }
}

/// NN for centeral brain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainNN {
    nn: BaseNN,
}

impl Default for BrainNN {
    fn default() -> Self {
        Self {
            nn: BaseNN::new_rand(
                Vec::from_iter(BRAIN_NN_SHAPE.into_iter().clone()),
                ACTIVATION_FUNCTION,
            ),
        }
    }
}

impl BrainNN {
    pub fn forward(&self, signal: &BrainSignal) -> Array1<f32> {
        self.nn.forward(signal.to_array())
    }

    pub fn get_rand_brain_output(&self) -> Array1<f32> {
        let mut rng = thread_rng();
        Array1::from_shape_fn((4,), |_| rng.gen::<f32>())
    }
}

impl fmt::Display for BrainNN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nn)
    }
}
