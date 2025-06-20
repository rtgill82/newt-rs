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
use newt::Component;
use newt::widgets::{Button,CompactButton};
use std::ptr;

#[test]
fn compact_button_create() {
    let button = CompactButton::new(-1, -1, "Ok");
    assert!(button.co() != ptr::null_mut());
}

#[test]
fn compact_button_partial_eq_true() {
    let button = CompactButton::new(-1, -1, "Ok");
    assert!(button == button);
}

#[test]
fn compact_button_partial_eq_false() {
    let button1 = CompactButton::new(-1, -1, "Ok");
    let button2 = Button::new(-1, -1, "Ok");
    assert!(button1 != button2);
}
