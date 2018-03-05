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

use amethyst::{State, Trans};
use amethyst::ecs::World;
use amethyst::renderer::Event;
use super::ActiveState;

pub struct SetupState;

impl State for SetupState {
    fn on_start(&mut self, _eng: &mut World) {}


    fn on_stop(&mut self, _eng: &mut World) {}


    fn on_pause(&mut self, _eng: &mut World) {}


    fn on_resume(&mut self, _eng: &mut World) {}


    fn handle_event(&mut self, _eng: &mut World, _event: Event) -> Trans {
        if false {
            Trans::Push(Box::new(ActiveState))
        } else {
            Trans::None
        }

    }

    fn fixed_update(&mut self, _eng: &mut World) -> Trans {
        Trans::None
    }

    fn update(&mut self, _eng: &mut World) -> Trans {
        Trans::None
    }
}