//
// Copyright (C) 2019 Robert Gill <rtgill82@gmail.com>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

#![allow(unused_imports)]
extern crate newt;
#[cfg(feature = "asm")]
use newt::grid::*;
use newt::prelude::*;

#[cfg(feature = "asm")]
pub fn main() {
    newt::init().unwrap();
    newt::cls();

    let rv;
    let l1 = Label::new(0, 0, "Hello");
    let l2 = Label::new(0, 0, "World");

    let stacked = HorizontalGrid::new(&[&l1, &l2]);
    let button_bar = ButtonBar::new(&["Yes", "No", "Maybe"]);

    let mut form = Form::new(None, 0);
    let mut grid = Grid::new(1, 2);
    grid.set_field(0, 0, &stacked, 1, 1, 1, 1, 0, 0);
    grid.set_field(0, 1, &button_bar, 1, 1, 1, 1, 0, 0);

    wrapped_window(&grid, "Grids");
    grid.add_to_form(&mut form).unwrap();
    rv = form.run().unwrap();
    newt::finished();

    for (i, button) in button_bar.buttons().iter().enumerate() {
        if rv == *button {
            println!("Button {} pressed.", i);
        }
    }
}

#[cfg(not(feature = "asm"))]
pub fn main() {
}
