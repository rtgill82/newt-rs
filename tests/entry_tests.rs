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
use newt::constants::FlagsSense;
use newt::Component;
use newt::widgets::{Button,Entry};
use std::ptr;

use newt::constants::COLORSET_ENTRY;
use newt::constants::COLORSET_DISENTRY;

#[test]
fn entry_create() {
    let entry = Entry::new(-1, -1, None, 10, 0);
    assert!(entry.co() != ptr::null_mut());
}

#[test]
fn entry_partial_eq_true() {
    let entry = Entry::new(-1, -1, None, 10, 0);
    assert!(entry == entry);
}

#[test]
fn entry_partial_eq_false() {
    let entry = Entry::new(-1, -1, None, 10, 0);
    let button = Button::new(-1, -1, "Ok");
    assert!(entry != button);
}

#[test]
fn entry_set_text() {
    let entry = Entry::new(-1, -1, None, 10, 0);
    let text = entry.get_text();
    assert!(text == "");

    entry.set_text("Hello world!", false);
    let text = entry.get_text();
    assert!(text == "Hello world!");
}

#[test]
fn entry_set_flags() {
    let entry = Entry::new(-1, -1, None, 10, 0);
    entry.set_flags(0, FlagsSense::Reset);
}

#[test]
fn entry_set_colors() {
    let entry = Entry::new(-1, -1, None, 10, 0);
    entry.set_colors(COLORSET_ENTRY, COLORSET_DISENTRY);
}

#[test]
fn entry_get_cursor_position() {
    let entry = Entry::new(-1, -1, None, 10, 0);
    assert!(entry.get_cursor_position() == 0);
}

#[test]
fn entry_set_cursor_position() {
    let entry = Entry::new(-1, -1, None, 10, 0);
    entry.set_cursor_position(5);
    assert!(entry.get_cursor_position() == 5);
}
