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

use amethyst::ecs::{ReadStorage, System, WriteStorage};
use ::component::{Position, Velocity};

pub struct BrainSystem {

}

impl<'a> System<'a> for BrainSystem {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut _pos, _vel): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        // for (pos, vel) in (&mut pos, &vel).join() {
        //     pos.0 += vel.0;
        // }
    }
}