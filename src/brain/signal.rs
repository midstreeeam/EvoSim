use bevy_rapier2d::na::{SMatrix, SVector};

use crate::consts::*;

const CL: usize = INWARD_NN_CHILDREN_INPUT_LEN;

pub struct SignalHandler{
    signal_vec: Vec<InwardNNInputSignalUnit>
}

impl Default for SignalHandler {
    fn default() -> Self {
        Self { 
            signal_vec: Vec::<InwardNNInputSignalUnit>::new()
        }
    }
}

impl SignalHandler {
    /// push inward signals and ids to handler
    pub fn push_inward(&mut self, signal: InwardNNInputSignal, nn_id:usize, parent_nn_id:Option<usize> ){
        self.signal_vec.push(
            InwardNNInputSignalUnit { signal: signal, nn_id: nn_id, parent_nn_id: parent_nn_id }
        )
    }

    // TODO: make it iterable so the output can be accessed better
    /// start neuron computing and return outputs
    pub fn run(&mut self){

    }

}

pub struct InwardNNInputSignalUnit{
    signal: InwardNNInputSignal,
    nn_id: usize,
    parent_nn_id: Option<usize>,
}

/// Input singal for single inward `BlockNeuron`
pub struct InwardNNInputSignal {
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

impl Default for InwardNNInputSignal {
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

impl InwardNNInputSignal {
    pub fn with_collision_signal(
        mut self,
        signal: Option<(bool,bool,[f32;2],f32)>
    ) -> Self {
        if let Some((
            wall,
            blob,
            vect,
            mag
        )) = signal{
            self.collision_with_wall = wall;
            self.collision_with_other_blob = blob;
            self.collision_vect = SVector::from_iterator(vect.into_iter());
            self.collision_mag = mag;
        }
        self
    }

    pub fn with_joint_singal(
        mut self,
        signal: (f32,f32,f32,f32)
    ) -> Self {
        let (motor_pos, motor_v, ang_pos, ang_v) = signal;
        self.cur_motor_pos = motor_pos;
        self.cur_motor_v = motor_v;
        self.joint_ang_pos = ang_pos;
        self.joint_ang_v = ang_v;
        self
    }
}
