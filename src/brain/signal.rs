use std::fmt;

use crate::{
    blob::block::{BlockDepth, ParentAnchor},
    consts::*,
};
use itertools::Itertools;
use ndarray::prelude::*;

const CL: usize = INWARD_NN_CHILDREN_INPUT_LEN;

// TODO: test correctness of signal
/// `SignalHandler` handles input signal from bevy
pub struct SignalHandler {
    pub inward_signal_vec: Vec<InwardNNInputSignalUnit>,
    pub brain_signal_vec: Vec<BrainSignalUnit>,
}

impl Default for SignalHandler {
    fn default() -> Self {
        Self {
            inward_signal_vec: Vec::<InwardNNInputSignalUnit>::new(),
            brain_signal_vec: Vec::<BrainSignalUnit>::new(),
        }
    }
}

impl SignalHandler {
    // TODO: use trait or other way to realise `len()`
    pub fn len(&self) -> usize {
        self.inward_signal_vec.len()
    }

    // /// stratify signals base on depth.
    // ///
    // /// Output order is positive-going,
    // /// which means groups with small depth have small index
    // ///
    // /// Side effect: `inward_signal_vec` will be sorted
    // pub fn stratify(&mut self) -> Vec<Vec<&mut InwardNNInputSignalUnit>> {
    //     // Sort by depth first
    //     self.inward_signal_vec.sort_by(|a, b| a.depth.cmp(&b.depth));

    //     // Group by depth
    //     self.inward_signal_vec
    //         .iter_mut()
    //         .group_by(|item| item.depth)
    //         .into_iter()
    //         .map(|(_, group)| group.collect())
    //         .collect()
    // }


    /// stratify signals base on depth.
    ///
    /// Output order is positive-going,
    /// which means groups with small depth have small index
    ///
    /// Side effect: `inward_signal_vec` will be sorted
    pub fn get_sig_mut(
        &mut self,
    ) -> (
        Vec<Vec<&mut InwardNNInputSignalUnit>>,
        Vec<&mut BrainSignalUnit>,
    ) {
        self.inward_signal_vec.sort_by(|a, b| a.depth.cmp(&b.depth));

        (
            self.inward_signal_vec
                .iter_mut()
                .group_by(|item| item.depth)
                .into_iter()
                .map(|(_, group)| group.collect())
                .collect(),
            self.brain_signal_vec.iter_mut().collect(),
        )
    }

    /// push inward signals and ids to handler
    pub fn push_inward(
        &mut self,
        signal: InwardNNInputSignal,
        nn_id: usize,
        parent_nn_id: usize,
        depth: &BlockDepth,
        anchor: &ParentAnchor,
    ) {
        self.inward_signal_vec.push(InwardNNInputSignalUnit {
            signal: signal,
            nn_id: nn_id,
            parent_nn_id: parent_nn_id,
            depth: depth.0 as usize,
            // inward nn must have parent anchor so unwarp
            anchor_pos: anchor.0.unwrap(),
        })
    }

    pub fn push_brain(&mut self, signal: BrainSignal, nn_id: usize) {
        self.brain_signal_vec.push(BrainSignalUnit {
            signal: signal,
            nn_id: nn_id,
        })
    }
}

pub struct InwardNNInputSignalUnit {
    pub signal: InwardNNInputSignal,
    pub nn_id: usize,
    pub parent_nn_id: usize,
    pub depth: usize,
    /// anchor point to parent
    pub anchor_pos: usize,
}

impl fmt::Debug for InwardNNInputSignalUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for InwardNNInputSignalUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}, Parent: {}, Depth: {}",
            self.nn_id, self.parent_nn_id, self.depth
        )
    }
}

impl InwardNNInputSignalUnit {
    pub fn get_signal_mut(&mut self) -> &mut InwardNNInputSignal {
        &mut self.signal
    }
}

/// Input singal for single inward `BlockNeuron`
pub struct InwardNNInputSignal {
    // collision signal
    collision_with_wall: bool,
    collision_with_other_blob: bool,
    collision_vect: [f32; 2],
    collision_mag: f32,

    // joint signal
    cur_motor_pos: f32,
    cur_motor_v: f32,
    joint_ang_pos: f32,
    joint_ang_v: f32,

    /// Input singal from children neurons.
    ///
    /// Order of children inputs depends on children's parent_anchor.
    children_input: Array2<f32>,
}

impl Default for InwardNNInputSignal {
    fn default() -> Self {
        Self {
            collision_with_wall: false,
            collision_with_other_blob: false,
            collision_vect: [0.0, 0.0],
            collision_mag: 0.0,
            cur_motor_pos: 0.0,
            cur_motor_v: 0.0,
            joint_ang_pos: 0.0,
            joint_ang_v: 0.0,
            children_input: Array2::<f32>::zeros((4, CL)),
        }
    }
}

impl InwardNNInputSignal {
    pub fn with_cf_signal(mut self, signal: Option<(bool, bool, [f32; 2], f32)>) -> Self {
        if let Some((wall, blob, vect, mag)) = signal {
            self.collision_with_wall = wall;
            self.collision_with_other_blob = blob;
            self.collision_vect = vect;
            self.collision_mag = mag;
        }
        self
    }

    pub fn with_joint_singal(mut self, signal: (f32, f32, f32, f32)) -> Self {
        let (motor_pos, motor_v, ang_pos, ang_v) = signal;
        self.cur_motor_pos = motor_pos;
        self.cur_motor_v = motor_v;
        self.joint_ang_pos = ang_pos;
        self.joint_ang_v = ang_v;
        self
    }

    pub fn push_child_signal(&mut self, signal: Array1<f32>, anchor: usize) {
        // anchor must in 0..=3
        match anchor {
            0..=3 => self
                .children_input
                .slice_mut(s![anchor, ..])
                .assign(&signal),
            _ => {
                panic!()
            }
        }
    }
}

/// Input signal of center block,
/// which do not have parent and joint
#[derive(Debug)]
pub struct BrainSignal {
    // collision signal
    collision_with_wall: bool,
    collision_with_other_blob: bool,
    collision_vect: [f32; 2],
    collision_mag: f32,

    /// input singal from children neurons.
    /// Shape is (4,CL)
    children_input: Array2<f32>,

    blob_mass_center: [f32; 2],
    blob_speed: [f32; 2],
}

impl Default for BrainSignal {
    fn default() -> Self {
        Self {
            collision_with_wall: false,
            collision_with_other_blob: false,
            collision_vect: [0.0, 0.0],
            collision_mag: 0.0,
            children_input: Array2::<f32>::zeros((4, CL)),
            blob_mass_center: [0.0, 0.0],
            blob_speed: [0.0, 0.0],
        }
    }
}

impl BrainSignal {
    pub fn with_cf_signal(mut self, signal: Option<(bool, bool, [f32; 2], f32)>) -> Self {
        if let Some((wall, blob, vect, mag)) = signal {
            self.collision_with_wall = wall;
            self.collision_with_other_blob = blob;
            self.collision_vect = vect;
            self.collision_mag = mag;
        }
        self
    }

    pub fn with_blob_info(mut self, center: [f32; 2], speed: [f32; 2]) -> Self {
        self.blob_mass_center = center;
        self.blob_speed = speed;
        self
    }

    pub fn push_child_signal(&mut self, signal: Array1<f32>, anchor: usize) {
        // anchor must in 0..=3
        match anchor {
            0..=3 => self
                .children_input
                .slice_mut(s![anchor, ..])
                .assign(&signal),
            _ => {
                panic!()
            }
        }
    }
}

pub struct BrainSignalUnit {
    pub signal: BrainSignal,
    pub nn_id: usize,
}

impl BrainSignalUnit {
    pub fn get_signal_mut(&mut self) -> &mut BrainSignal {
        &mut self.signal
    }
}

impl fmt::Debug for BrainSignalUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for BrainSignalUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Brain:  ID: {}",
            self.nn_id
        )
    }
}