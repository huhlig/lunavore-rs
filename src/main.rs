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
extern crate amethyst;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod component;
mod generator;
mod network;
mod states;
mod system;

use amethyst::ApplicationBuilder;
use component::{Position, Radiance, Velocity};
use states::SetupState;
use std::error::Error;

fn run() -> Result<(), Box<Error>> {
    let mut builder =
        ApplicationBuilder::new("./", SetupState)?;
    builder.world.register::<Position>();
    builder.world.register::<Radiance>();
    builder.world.register::<Velocity>();
    let mut application = builder.build()?;

    Ok(application.run())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
        ::std::process::exit(1);
    }
}
