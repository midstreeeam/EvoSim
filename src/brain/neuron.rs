use crate::consts::*;

const CL: usize = INWARD_NN_CHILDREN_INPUT_LEN;

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
}
