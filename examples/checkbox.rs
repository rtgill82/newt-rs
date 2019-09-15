//
// Copyright (C) 2019  Robert Gill <locke@sdf.lonestar.org>
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

    let cb1 = Checkbox::new(4, 1, "Option 1", None, None);
    let cb2 = Checkbox::new(4, 2, "Option 2", Some('X'), Some(&[' ', 'X', '*']));
    let ok = CompactButton::new(7, 4, "Ok");

    let mut form = Form::new(None, 0);
    form.add_components(&[&cb1, &cb2, &ok]).unwrap();
    form.run().unwrap();
    newt::finished();

    println!("Option 1: {}", cb1.get_value());
    println!("Option 2: {}", cb2.get_value());
}
