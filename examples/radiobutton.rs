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

extern crate newt;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 6, Some("Options")).unwrap();

    // Create the first `Radiobutton` in the set, set as default.
    let radio1 = Radiobutton::new(4, 1, "Option 1", true, None);

    // Create the second `Radiobutton in the set, adding `radio1` as the
    // previous button.
    let radio2 = Radiobutton::new(4, 2, "Option 2", false,
                                  Some(&radio1));

    // Create the third `Radiobutton` in the set, adding `radio2` as the
    // previous button.
    let radio3 = Radiobutton::new(4, 3, "Option 3", false,
                                  Some(&radio2));

    let ok = CompactButton::new(7, 5, "Ok");

    let mut form = Form::new(None, 0);
    form.add_components(&[&radio1, &radio2, &radio3, &ok]).unwrap();
    form.run().unwrap();
    newt::finished();

    let buttons = [(&radio1, "Option 1"),
                   (&radio2, "Option 2"),
                   (&radio3, "Option 3")];

    // Get the currently selected `Radiobutton` from the first in the set
    // (any will do).
    let current = radio1.get_current().unwrap();

    // Find the currently selected `Radiobutton` in the array of available
    // buttons.
    for val in buttons.iter() {
        let &(radio, text) = val;
        if *radio == current {
            println!("Selected Option: {}", text);
        }
    }
}
