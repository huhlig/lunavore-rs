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
//  with Lunavore. If not, see <http://www.gnu.org/licenses/>.
//

use std::f64::consts::E;

pub type Activation = fn(f64) -> f64;

pub fn identity(x: f64) -> f64 { x }

pub fn binary_step(x: f64) -> f64 { if x < 0.0 { 0.0 } else { 1.0 } }

pub fn logistic(x: f64) -> f64 { 1.0 / (1.0 + E.powf(-x)) }

pub fn tanh(x: f64) -> f64 { (E.powf(x) - E.powf(-x)) / (E.powf(x) + E.powf(-x)) }