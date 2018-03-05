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

use rand::{self, Rng};
use super::activation::Activation;

type Weight = f64;

pub struct Neuron {
    weights: Vec<Weight>
}

pub struct Layer {
    neurons: Vec<Neuron>
}

pub struct Network {
    activation: Activation,
    layers: Vec<Layer>,
    inputs: usize,
}

impl Network {
    pub fn new(layer_sizes: &[usize], activation: Activation) -> Result<NeuralNetwork, Box<Error>> {
        let mut rng = rand::thread_rng();

        if layer_sizes.len() < 2 {
            return Err("You must have at least 2 layers");
        }

        for &layer_size in layer_sizes.iter() {
            if layer_size < 1 {
                return Err("You cannot have empty layers");
            }
        }

        let mut layers = Vec::new();
        let mut layer_iter = layers_sizes.iter();

        // Get number of Inputs
        let input_count = *layer_iter.next().unwrap();
        let mut prev_neuron_count = input_count;

        let mut network = Network {
            activation: activation,
            layers: Vec::with_capacity(layer_sizes - 1),
            inputs: input_count,
        };

        // setup the rest of the layers

        for &layer_size in layer_iter {
            let mut layer = Layer {
                neurons: Vec::with_capacity(layer_size),
            };
            for _ in 0..layer_size {
                let mut neuron = Perceptron {
                    weights: Vec::with_capacity(prev_neuron_count + 1),
                };
                for _ in 0..prev_neuron_count + 1 {
                    node.push(rng.gen_range(-0.5, 0.5));
                }
                layer.neurons.push(node);
            }
            network.layers.push(layer);
            prev_neuron_count = layer_size;
        }
        return network;
    }
}