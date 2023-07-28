use std::{f32::consts::PI, time::Duration};

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
pub const WORLD_WIDTH: f32 = 10000.0;
pub const WORLD_HEIGHT: f32 = 10000.0;

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

// mutate
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
/// porbablity of an single nn to mutate
pub const MUTATE_NN_PORB: f32 = 0.5;
/// standard deviation for normal distribution mutation
pub const MUTATE_NN_STD: f32 = 0.1;

// io
pub const EXPORT_PATH: &'static str = "./export/";
/// temperary
pub const LOAD_FNAME: &'static str = "./export/2023-07-25T15-28-56.json";