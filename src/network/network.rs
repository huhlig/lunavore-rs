//
//  This file is part of 'Lunavore'.
//
//  Lunavore is free software: you can redistribute it and/or modify it under
//  the terms of the GNU General Public License as published by the Free
//  Software Foundation, either version 3 of the License, or  (at your option)
//  any later version.
//
//  Lunavore is distributed in the hope that it will be useful, but WITHOUT
//  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
//  FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
//  more details.
//
//  You should have received a copy of the GNU General Public License along
//  with Lunavore.  If not, see <http://www.gnu.org/licenses/>.
//

// https://github.com/jackm321/RustNN/blob/master/src/lib.rs

use std::error::Error;
use rand::{self, Rng};
use super::activation::Activation;
use super::error::CreationError;

pub type Weight = f64;

#[derive(Debug, Serialize, Deserialize)]
pub struct Neuron {
    weights: Vec<Weight>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    neurons: Vec<Neuron>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeuralNetwork {
    activation: Activation,
    layers: Vec<Layer>,
    inputs: usize,
}

impl NeuralNetwork {
    pub fn new(layer_sizes: &[usize], activation: Activation) -> Result<NeuralNetwork, Box<Error>> {
        let mut rng = rand::thread_rng();

        if layer_sizes.len() < 2 {
            return Err(Box::new(CreationError::InsufficientLayers));
        }

        for &layer_size in layer_sizes.iter() {
            if layer_size < 1 {
                return Err(Box::new(CreationError::InsufficientNodes));
            }
        }

        let mut layer_iter = layer_sizes.iter();

        // Get number of Inputs
        let input_count = *layer_iter.next().unwrap();
        let mut prev_neuron_count = input_count;

        let mut network = NeuralNetwork {
            activation: activation,
            layers: Vec::with_capacity(layer_sizes.len() - 1),
            inputs: input_count,
        };

        // setup the rest of the layers

        for &layer_size in layer_iter {
            println!("{}",layer_size);
            let mut layer = Layer {
                neurons: Vec::with_capacity(layer_size),
            };
            for _ in 0..layer_size {
                let mut neuron = Neuron {
                    weights: Vec::with_capacity(prev_neuron_count + 1),
                };
                for _ in 0..prev_neuron_count {
                    neuron.weights.push(rng.gen_range(-0.5, 0.5));
                }
                layer.neurons.push(neuron);
            }
            network.layers.push(layer);
            prev_neuron_count = layer_size;
        }

        return Ok(network);
    }
}

