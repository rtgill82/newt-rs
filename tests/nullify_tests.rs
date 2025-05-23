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

#![cfg(all(feature = "asm",
           any(target_arch = "aarch64", target_arch = "arm",
               target_arch = "riscv32", target_arch = "riscv64",
               target_arch = "x86",     target_arch = "x86_64")))]
extern crate newt;
use newt::grid::*;
use newt::prelude::*;

#[test]
#[should_panic]
fn test_form_nullify() {
    let button = Button::new(0, 0, "Ok");
    {
        let mut form = Form::new(None, 0);
        form.add_component(&button).unwrap();
    }
    button.get_position();
}

#[test]
#[should_panic]
fn test_grid_nullify() {
    let button = Button::new(0, 0, "Ok");
    let mut grid = Grid::new(1, 2);
    {
        grid.set_field(0, 0, &button, 1, 1, 1, 1, 0, 0);
        let mut form = Form::new(None, 0);
        grid.add_to_form(&mut form).unwrap();
    }
    button.get_position();
}

#[test]
#[should_panic]
fn test_deep_grid_nullify() {
    let button = Button::new(0, 0, "Ok");
    let hgrid = HorizontalGrid::new(&[&button]);
    {
        let mut grid = Grid::new(1, 2);
        let mut form = Form::new(None, 0);

        grid.set_field(0, 0, &hgrid, 1, 1, 1, 1, 0, 0);
        grid.add_to_form(&mut form).unwrap();
    }
    button.get_position();
}
