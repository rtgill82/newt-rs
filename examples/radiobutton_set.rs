//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
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

extern crate newt;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 6, Some("Options")).unwrap();

    let mut set = RadiobuttonSet::new();
    set.add_radiobutton(4, 1, "Option 1");
    set.add_radiobutton(4, 2, "Option 2");
    set.add_radiobutton(4, 3, "Option 3");
    let ok = CompactButton::new(7, 5, "Ok");

    let mut form = Form::new(None, 0);
    set.add_to_form(&mut form).unwrap();
    form.add_component(&ok).unwrap();

    form.run().unwrap();
    newt::finished();

    println!("Selected Option: {}", set.get_current()+1);
}
