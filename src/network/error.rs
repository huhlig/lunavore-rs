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

use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum CreationError {
    InsufficientNodes,
    InsufficientLayers,
}

impl Display for CreationError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.description())
    }
}

impl Error for CreationError {
    fn description(&self) -> &str {
        match self {
            &CreationError::InsufficientLayers => "Network MUST have a minimum of 2 layers.",
            &CreationError::InsufficientNodes => "Each layer MUST have a minimum of one node.",
        }
    }

    fn cause(&self) -> Option<&Error> { Option::None }
}