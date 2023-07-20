use std::fmt;

use ndarray::prelude::*;
use rand::{distributions::Uniform, prelude::Distribution};

#[derive(Debug, Clone)]
pub enum Activation {
    ReLU,
    Sigmoid,
}

impl Activation {
    fn apply(&self, input: f32) -> f32 {
        match self {
            Activation::ReLU => input.max(0.0),
            Activation::Sigmoid => 1.0 / (1.0 + (-input).exp()),
        }
    }
}

#[derive(Debug, Clone)]
struct BaseLayer {
    weights: Array2<f32>,
    bias: Array1<f32>
}

impl BaseLayer {
    fn new_rand(nodes_in: usize, nodes_out: usize) -> BaseLayer {
        let weight_dist = Uniform::new(-1.0, 1.0);
        let bias_dist = Uniform::new(-1.0, 1.0);

        let weights = Array::from_shape_fn((nodes_out, nodes_in), |_| weight_dist.sample(&mut rand::thread_rng()));
        let bias = Array::from_shape_fn(nodes_out, |_| bias_dist.sample(&mut rand::thread_rng()));

        BaseLayer { weights, bias }
    }

    fn new_empty(nodes_in: usize, nodes_out: usize) -> BaseLayer {
        let weights = Array2::<f32>::zeros((nodes_out,nodes_in));
        let bias = Array1::<f32>::zeros(nodes_out);
        BaseLayer { weights, bias }
    }

    fn forward(&self, input: &Array1<f32>, activation: &Activation) -> Array1<f32> {
        assert_eq!(input.len(),self.weights.shape()[1]);
        let z = self.weights.dot(input) + &self.bias;
        z.mapv(|x| activation.apply(x))
    }
}

impl fmt::Display for BaseLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Weights shape: {:?}, Bias shape: {:?}",
            self.weights.dim(),
            self.bias.dim()
        )
    }
}

#[derive(Debug, Clone)]
pub struct BaseNN {
    layers: Vec<BaseLayer>,
    activation: Activation
}

impl BaseNN {
    pub fn new_rand (layer_sizes:Vec<usize>, activation: Activation) -> Self {
        let mut layers = Vec::<BaseLayer>::new();
        if layer_sizes.len() <= 1 {
            panic!()
        }
        for i in 1..layer_sizes.len(){
            layers.push(BaseLayer::new_rand(layer_sizes[i-1], layer_sizes[i]));
        }
        Self { layers, activation }
    }

    pub fn new_empty (layer_sizes:Vec<usize>, activation: Activation) -> Self {
        let mut layers = Vec::<BaseLayer>::new();
        if layer_sizes.len() <= 1 {
            panic!()
        }
        for i in 1..layer_sizes.len(){
            layers.push(BaseLayer::new_empty(layer_sizes[i-1], layer_sizes[i]));
        }
        Self { layers, activation }
    }

    pub fn forward(&self, mut input: Array1<f32>) -> Array1<f32> {
        // println!("{}",input.len());
        for layer in &self.layers {
            input = layer.forward(&input, &self.activation);
        }
        input
    }
}

impl fmt::Display for BaseNN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let layers_str: Vec<String> = self.layers.iter().map(|layer| format!("{}", layer)).collect();
        write!(
            f,
            "Neural Network Structure:\nActivation: {:?}\nLayers:\n{}",
            self.activation,
            layers_str.join("\n-----\n")
        )
    }
}