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
    newt::centered_window(15, 6, Some("Options")).unwrap();

    let listbox: Listbox = Listbox::new(1, 1, 3, FLAG_MULTIPLE);
    let ok = CompactButton::new(1, 5, "Ok");
    let clear = CompactButton::new(6, 5, "Clear");

    for i in 1..10 {
        let text = format!("Entry {}", i);
        listbox.append_entry(&text, i).unwrap();
    }

    let mut form = Form::new(None, 0);
    form.add_components(&[&listbox, &ok, &clear]).unwrap();

    while form.run().unwrap() == clear { listbox.clear(); }
    newt::finished();

    let current = listbox.get_current();
    let selected = listbox.get_selection();
    println!("current = {:?}", current);
    println!("selected = {:?}", selected);
}
