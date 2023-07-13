
/// neuron for blocks
#[derive(Debug)]
pub struct BlockNeuron{
    pub value: f32
}

impl Default for BlockNeuron{
    fn default() -> Self {
        todo!()
    }
}

impl BlockNeuron{
    pub fn new() -> Self{
        Self { value: 0.0 }
    }

    pub fn thread_test(&self){
        let a = std::thread::current(); 
        println!("{}",a.name().unwrap());
    }
}