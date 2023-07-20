use crate::brain::nn::Activation;

// timestep
pub const RAPIER_DT:f32 = 1.0/60.0;
pub const RAPIER_SUBSTEPS:usize = 1;

// scale world size
pub const WORLD_WIDTH:f32 = 10000.0;
pub const WORLD_HEIGHT:f32 = 10000.0;

// joint config
pub const MOTOR_STIFFNESS:f32 = 10.0;
pub const MOTOR_DAMPING:f32 = 0.0;
pub const ENABLE_CONTACTS:bool = false;
// joint contorl
pub const MOTOR_MAX_TARGET_V:f32 = 3.0;

// math
pub const EPSILON:f32 = 0.0001; // max error

// physics
pub const DRAG_COEFF:f32 = 5.0; // drag coefficient in fluid simulation
pub const DEFAULT_DENSITY:f32 = 1.0;

// Geno
pub const GENO_MAX_DEPTH:u32 = 2; // max recursion depth of Geno type
pub const DEFAULT_BLOCK_SIZE:[f32;2] = [50.0,50.0];

// Rand
pub const RAND_NODE_NOT_NONE:f64 = 0.9;
pub const RAND_SIZE_SCALER:[f32;2] = [0.5,2.0];

// nn
/// each children has 4 input values during inward pass
/// 
/// shape is for ndarray
pub const INWARD_NN_CHILDREN_INPUT_LEN:usize = 4;
/// each parent passes 4 value to children in outward pass
pub const OUTWARD_NN_PARENT_INPUT_LEN:usize = 4;
/// currently it has 3 layers, the hidden layer has 8 nodes
pub const INWARD_NN_SHAPE:[usize;3] = [
    // input layer
    INWARD_NN_CHILDREN_INPUT_LEN*4 + 9,
    // hidden layer
    8, 
    // output layer
    INWARD_NN_CHILDREN_INPUT_LEN
];
/// outward nn shape
pub const OUTWARD_NN_SHAPE:[usize;3] = [
    OUTWARD_NN_PARENT_INPUT_LEN + 9,
    8,
    OUTWARD_NN_PARENT_INPUT_LEN + 2
];
/// brain nn shape
pub const BRAIN_NN_SHAPE:[usize;3] = [
    INWARD_NN_CHILDREN_INPUT_LEN*4+9,
    8,
    OUTWARD_NN_PARENT_INPUT_LEN
];
/// activation function
/// 
/// ReLU will make all output positive
pub const ACTIVATION_FUNCTION:Activation = Activation::Sigmoid;