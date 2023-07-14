use std::f32::consts::PI;

use bevy_rapier2d::na::{SMatrix, SVector};
use rand::prelude::*;

use crate::consts::*;

const CL: usize = INWARD_NN_CHILDREN_INPUT_LEN;

/// Input singal for single inward `BlockNeuron`
pub struct InwardNNInputSingal {
    // collision signal
    collision_with_wall: bool,
    collision_with_other_blob: bool,
    collision_vect: SVector<f32, 2>,
    collision_mag: f32,

    // joint signal
    cur_motor_pos: f32,
    cur_motor_v: f32,
    joint_ang_pos: f32,
    joint_ang_v: f32,

    /// input singal from children neurons
    children_input: SMatrix<f32, 3, CL>,
}

impl Default for InwardNNInputSingal {
    fn default() -> Self {
        Self {
            collision_with_wall: false,
            collision_with_other_blob: false,
            collision_vect: SVector::<f32, 2>::zeros(),
            collision_mag: 0.0,
            cur_motor_pos: 0.0,
            cur_motor_v: 0.0,
            joint_ang_pos: 0.0,
            joint_ang_v: 0.0,
            children_input: SMatrix::<f32, 3, CL>::zeros(),
        }
    }
}

impl InwardNNInputSingal {
    pub fn with_collision_signal(
        mut self,
        wall: bool,
        blob: bool,
        vect: [f32; 2],
        mag: f32,
    ) -> Self {
        self.collision_with_wall = wall;
        self.collision_with_other_blob = blob;
        self.collision_vect = SVector::from_iterator(vect.into_iter());
        self.collision_mag = mag;
        self
    }

    pub fn with_joint_singal(
        mut self,
        motor_pos: f32,
        motor_v: f32,
        ang_pos: f32,
        ang_v: f32,
    ) -> Self {
        self.cur_motor_pos = motor_pos;
        self.cur_motor_v = motor_v;
        self.joint_ang_pos = ang_pos;
        self.joint_ang_v = ang_v;
        self
    }
}

/// neuron for blocks.
///
/// Each block should have two independent neurons:
/// InwardNN and OutwardNN
#[derive(Debug)]
pub struct GenericBlockNN {
    pub value: f32,
}

impl Default for GenericBlockNN {
    fn default() -> Self {
        todo!()
    }
}

impl GenericBlockNN {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn thread_test(&self) {
        let a = std::thread::current();
        println!("{}", a.name().unwrap());
    }

    /// output the motor target pos and motor target v
    /// Takes input layer's singal
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
