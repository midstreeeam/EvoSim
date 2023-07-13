use std::f32::consts::PI;

use rand::prelude::*;

use crate::consts::MOTOR_MAX_TARGET_V;

/// neuron for blocks
#[derive(Debug)]
pub struct BlockNeuron {
    pub value: f32,
}

impl Default for BlockNeuron {
    fn default() -> Self {
        todo!()
    }
}

impl BlockNeuron {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn thread_test(&self) {
        let a = std::thread::current();
        println!("{}", a.name().unwrap());
    }

    /// output the motor target pos and motor target v
    pub fn get_output(&self) -> [f32; 2] {
        todo!()
    }

    // output random contorl signal
    pub fn get_rand_output(&self) -> [f32; 2] {
        let mut rng = thread_rng();
        [
            rng.gen_range(-PI..PI),
            rng.gen_range(-MOTOR_MAX_TARGET_V..MOTOR_MAX_TARGET_V),
        ]
    }
}
