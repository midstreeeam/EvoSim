//! all the consts

use std::{f32::consts::PI, time::Duration};

use bevy::prelude::KeyCode;

use crate::brain::nn::Activation;

/// thread count
/// 
/// default is automatic
pub const THREAD_COUNT:usize = 8;

// timestep
pub const RAPIER_DT: f32 = 1.0 / 60.0;
pub const RAPIER_SUBSTEPS: usize = 1;

// debug
pub const PRINT_FUNCTION_TIME:bool = false;
/// min time cost each frame to be print
pub const MIN_PRINT_DURATION:Duration = Duration::from_micros(500);

// scale world size
// train walk
pub const WORLD_WIDTH_WALK: f32 = 100000.0;
pub const WORLD_HEIGHT_WALK: f32 = 2000.0;
// train swim
pub const WORLD_WIDTH_SWIM: f32 = 10000.0;
pub const WORLD_HEIGHT_SWIM: f32 = 10000.0;

// joint config
pub const MOTOR_STIFFNESS: f32 = 10.0;
pub const MOTOR_DAMPING: f32 = 0.0;
pub const ENABLE_CONTACTS: bool = false;
// joint contorl
pub const MOTOR_MAX_TARGET_V: f32 = 3.0;
// joint motor boundry
// not use currently since using sigmoid
pub const MAX_MOTOR_POS_ABS: f32 = PI;
pub const MAX_MOTOR_VEL_ABS: f32 = 1.0;

// math
pub const EPSILON: f32 = 0.0001; // max error
pub const POSITION_EPSILON: f32 = 0.001; // max eorror in position validation
pub const PANIC_TRY_TIMES: usize = 10000;

// physics
pub const DRAG_COEFF: f32 = 1.0; // drag coefficient in fluid simulation
pub const DEFAULT_DENSITY: f32 = 1.0;
pub const DEFAULT_DAMPING_LINEAR: f32 = 0.0;
pub const DEFAULT_DAMPING_ANGULAR: f32 = 2.0;

// Geno
pub const GENO_MAX_DEPTH: u32 = 3; // max recursion depth of Geno type
pub const DEFAULT_BLOCK_SIZE: [f32; 2] = [50.0, 50.0];

// Rand
pub const RAND_NODE_NOT_NONE: f64 = 0.9;
pub const RAND_SIZE_SCALER: [f32; 2] = [0.5, 2.0];

// nn
/// each children has 4 input values during inward pass
///
/// shape is for ndarray
pub const INWARD_NN_CHILDREN_INPUT_LEN: usize = 4;
/// each parent passes 4 value to children in outward pass
pub const OUTWARD_NN_PARENT_INPUT_LEN: usize = 4;
/// currently it has 3 layers, the hidden layer has 8 nodes
pub const INWARD_NN_SHAPE: [usize; 3] = [
    // input layer
    INWARD_NN_CHILDREN_INPUT_LEN * 4 + 9,
    // hidden layer
    8,
    // output layer
    INWARD_NN_CHILDREN_INPUT_LEN,
];
/// outward nn shape
pub const OUTWARD_NN_SHAPE: [usize; 3] = [
    OUTWARD_NN_PARENT_INPUT_LEN + 9,
    8,
    OUTWARD_NN_PARENT_INPUT_LEN + 2,
];
/// brain nn shape
pub const BRAIN_NN_SHAPE: [usize; 3] = [
    INWARD_NN_CHILDREN_INPUT_LEN * 4 + 9,
    8,
    OUTWARD_NN_PARENT_INPUT_LEN,
];
/// activation function
///
/// ReLU will make all output positive
pub const ACTIVATION_FUNCTION: Activation = Activation::Sigmoid;

#[cfg(feature = "demo")]
// mutate for demo
pub mod mutate_consts{
    use std::f32::consts::PI;
    /// probablity of having tree structure mutate
    /// 
    /// if the tree structure is going to mutate, maximumly 1 node will mutate
    /// since single node blob can't lose a node anymore
    pub const MUTATE_TREE_STRUCTURE_PROB: f32 = 0.9;
    /// probablity for the choosen node to gain a child node,
    /// otherwise the blob gonna lose a limb
    pub const MUTATE_GAIN_LIMB_PROB: f32 = 0.5;
    /// max times to retry to add a new limb if last one cause self-conflict
    /// 
    /// condition of impossible new limb exist (the parent indicator was dropped)
    pub const MUTATE_GAIN_LIMB_MAX_TRY: u32 = 10;
    /// probablity of having limb size mutate
    pub const MUTATE_BLOCK_SIZE_PROB: f32 = 1.0;
    /// probablity for each signle block to mutate
    /// 
    /// mutation is not garenteed since it might cause self-confliction
    pub const MUTATE_SINGLE_BLOCK_SIZE_PROB: f32 = 0.5;
    /// scaler for block mutation
    pub const MUTATE_SINGLE_BLOCK_SIZE_SCALER: [f32;2] = [0.9,1.1];
    /// clamp between this scaler for `DEFAULT_BLOCK_SIZE`
    pub const MUTATE_SINGLE_BLOCK_SIZE_CLAMP_SCALER: [f32;2] = [0.5,2.0];
    /// porbablity of a signle joint limit to mutate
    pub const MUTATE_JOINT_LIMIT_PROB: f32 = 0.5;
    pub const MUTATE_JOINT_LIMIT_MIN: f32 = -PI*0.9;
    pub const MUTATE_JOINT_LIMIT_MAX: f32 = PI*0.9;
    /// porbablity of a single nn to mutate
    pub const MUTATE_NN_PORB: f32 = 0.5;
    /// standard deviation for normal distribution mutation
    pub const MUTATE_NN_STD: f32 = 0.1;
    /// probablity of a single weight to mutate after the `BaseNN` is chosen to be mutate.
    pub const MUTATE_NN_WEIGHT_PROB: f32 = 0.8;
    /// probablity of a single bias to mutate after the `BaseNN` is chosen to be mutate.
    pub const MUTATE_NN_BIAS_PROB: f32 = 0.8;
}

#[cfg(feature = "move")]
// mutate for move training
pub mod mutate_consts{
    use std::f32::consts::PI;
    /// probablity of having tree structure mutate
    /// 
    /// if the tree structure is going to mutate, maximumly 1 node will mutate
    /// since single node blob can't lose a node anymore
    pub const MUTATE_TREE_STRUCTURE_PROB: f32 = 0.05;
    /// probablity for the choosen node to gain a child node,
    /// otherwise the blob gonna lose a limb
    pub const MUTATE_GAIN_LIMB_PROB: f32 = 0.5;
    /// max times to retry to add a new limb if last one cause self-conflict
    /// 
    /// condition of impossible new limb exist (the parent indicator was dropped)
    pub const MUTATE_GAIN_LIMB_MAX_TRY: u32 = 10;
    /// probablity of having limb size mutate
    pub const MUTATE_BLOCK_SIZE_PROB: f32 = 0.25;
    /// probablity for each signle block to mutate
    /// 
    /// mutation is not garenteed since it might cause self-confliction
    pub const MUTATE_SINGLE_BLOCK_SIZE_PROB: f32 = 0.5;
    /// scaler for block mutation
    pub const MUTATE_SINGLE_BLOCK_SIZE_SCALER: [f32;2] = [0.7,1.3];
    /// clamp between this scaler for `DEFAULT_BLOCK_SIZE`
    pub const MUTATE_SINGLE_BLOCK_SIZE_CLAMP_SCALER: [f32;2] = [0.3,2.0];
    /// porbablity of a signle joint limit to mutate
    pub const MUTATE_JOINT_LIMIT_PROB: f32 = 0.1;
    pub const MUTATE_JOINT_LIMIT_MIN: f32 = -PI*0.9;
    pub const MUTATE_JOINT_LIMIT_MAX: f32 = PI*0.9;
    /// porbablity of a single nn to mutate
    pub const MUTATE_NN_PORB: f32 = 0.25;
    /// standard deviation for normal distribution mutation
    pub const MUTATE_NN_STD: f32 = 0.15;
    /// probablity of a single weight to mutate after the `BaseNN` is chosen to be mutate.
    pub const MUTATE_NN_WEIGHT_PROB: f32 = 0.8;
    /// probablity of a single bias to mutate after the `BaseNN` is chosen to be mutate.
    pub const MUTATE_NN_BIAS_PROB: f32 = 0.8;
}

// training
/// survival rate in `train_move.rs`
pub const TRAIN_MOVE_SURVIVAL_RATE: f32 = 0.5;
/// population for each training iteration
pub const POPULATION: usize = 30;
/// limit for population generation area
/// 
/// 100*100 world size with 0.5 ratio result in 50*50 generation area
pub const SCATTER_RATIO_Y: f32 = 0.8;
pub const SCATTER_RATIO_X: f32 = 0.8;

/// min distance between two spawn point
pub const BLOB_SPAWN_POINT_RADIUS: f32 = 750.0;
/// how long a signle iteration, counted in frame
pub const ITERATION_LENGTH: usize = 1000;
pub const CHECKPOINTS_LENGTH: usize = 100;
/// tournament selection hybrid
pub const HYBRID_RATE: f32 = 0.3;
/// choose between swim and walk
pub const TRAINING_MODE: &'static str = "swim";

// io
pub const EXPORT_PATH: &'static str = "./export/";
pub const LOAD_FOLDER: &'static str = "./export/";
pub const LOAD_FNAME: &'static str = "./export/2023-07-25T15-28-56.json";
pub const LOAD_NEWEST_FILE: bool = true;

// user contorl
pub const MUTATE_AND_REFRESH_KEYCODE: KeyCode = KeyCode::M;
pub const NEW_ITERATION_KEYCODE: KeyCode = KeyCode::R;
pub const AUTO_NO_VSYNC_KEYCODE: KeyCode = KeyCode::V;
pub const SAVE_ALL_BLOBS_TO_JSON: KeyCode = KeyCode::S;
pub const LOAD_ALL_BLOBS_FROM_JSON: KeyCode = KeyCode::L;
pub const CLEAN_ALL_BLOBS_KEYCODE: KeyCode = KeyCode::X;

// log
pub const LOG_PATH: &'static str = "./run.log";