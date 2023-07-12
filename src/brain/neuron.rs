
/// neuron for blocks
pub struct BlockNeuron{
    pub value: f32
}

impl BlockNeuron{
    pub fn thread_test(&self){
        let a = std::thread::current(); 
        println!("{}",a.name().unwrap());
    }
}