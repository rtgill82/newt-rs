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
use newt::callbacks::EntryFilter;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(22, 5, None).unwrap();

    // last character entered
    let mut g_ch: char = '\0';
    // last cursor position
    let mut g_cursor: i32 = 0;
    // user data from last Entry modified
    let mut g_data: i32 = 0;

    // Create closure to be used to filter the Entry field.
    let mut f = |_e: &Entry, data: Option<&i32>, ch: char, cursor: i32| {
        g_data = *data.unwrap(); // set user data
        g_ch = ch;               // set character entered
        g_cursor = cursor;       // set cursor position

        // The returned character gets added to the entry.
        // If for example you want to fill an entry with asterisks despite
        // what the user entered, then return '*' here.
        return ch; // Return the entered character.
    };

    let l1 = Label::new(1, 1, "Entry 1:");
    let l2 = Label::new(1, 2, "Entry 2:");
    let e1 = Entry::new(10, 1, None, 10, 0);
    let e2 = Entry::new(10, 2, None, 10, 0);
    let ok = CompactButton::new(7, 4, "Ok");

    let components: &[&dyn Component] = &[&l1, &l2, &e1, &e2, &ok];
    let mut form = Form::new(None, 0);
    form.add_components(components).unwrap();

    // Filter the first Entry, passing user data `5`.
    let mut filter = EntryFilter::new(&e1, Some(5), &mut f);
    // Filter the second Entry, passing user data `10`.
    filter.add_entry(&e2, Some(10));

    form.run().unwrap();
    newt::finished();

    println!("Entry 1: {}", e1.get_text());
    println!("Entry 2: {}", e2.get_text());

    // Display the last values set by the filter.
    println!("ch = {}", g_ch);
    println!("cursor = {}", g_cursor);
    println!("data = {}", g_data);
}
