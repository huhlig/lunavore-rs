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
//!
//!
//!

mod activation;
mod error;
mod network;

pub use self::activation::Activation;
pub use self::network::{Weight, Neuron, Layer, NeuralNetwork};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_creation() {
        let brain = NeuralNetwork::new(&[4, 4], Activation::Identity)
            .expect("Unable to generate NeuralNetwork");

        let str = serde_json::to_string(&brain).expect("Unable to Serialize");

        println!("NeuralNetwork: {}", str);
    }
}