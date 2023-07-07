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

// math
pub const EPSILON:f32 = 0.0001; // max error

// physics
pub const DRAG_COEFF:f32 = 5.0; // drag coefficient in fluid simulation
pub const DEFAULT_DENSITY:f32 = 1.0;