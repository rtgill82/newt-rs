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
    newt::centered_window(20, 9, Some("Options")).unwrap();

    let tree: CheckboxTree =
        CheckboxTree::new(0, 0, 7, Some(&[' ', 'A', 'B']), 0);
    let ok = CompactButton::new(7, 8, "Ok");

    tree.add_item("Tree 1", 1, 0, Some(&[0]));
    tree.add_item("Option 1", 2, 0, Some(&[0, ARG_APPEND]));
    tree.add_item("Option 2", 3, 0, Some(&[0, ARG_APPEND]));
    tree.add_item("Tree 2", 4, 0, Some(&[1]));
    tree.add_item("Option 3", 5, 0, Some(&[1, ARG_APPEND]));
    tree.add_item("Option 4", 6, 0, Some(&[1, ARG_APPEND]));
    tree.add_item("Option 5", 7, 0, Some(&[1, ARG_APPEND]));

    let mut form = Form::new(None, 0);
    form.add_components(&[&tree, &ok]).unwrap();
    form.run().unwrap();
    newt::finished();

    let selection = tree.get_selection();
    println!("selection: {:?}", selection);
    for i in selection.iter() {
        println!("{} is set to {}", i, tree.get_entry_value(*i));
    }
}
