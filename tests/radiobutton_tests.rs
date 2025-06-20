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
use newt::widgets::{Button,Radiobutton};
use std::ptr;

#[test]
fn radiobutton_create() {
    let radio = Radiobutton::new(-1, -1, "Yes", true, None);
    assert!(radio.co() != ptr::null_mut());
}

#[test]
fn radiobutton_partial_eq_true() {
    let radio = Radiobutton::new(-1, -1, "Yes", true, None);
    assert!(radio == radio);
}

#[test]
fn radiobutton_partial_eq_false() {
    let radio = Radiobutton::new(-1, -1, "Yes", true, None);
    let button = Button::new(-1, -1, "Ok");
    assert!(radio != button);
}

#[test]
fn radiobutton_get_current() {
    let radio = Radiobutton::new(-1, -1, "Yes", true, None);
    assert!(radio.get_current().unwrap() == radio);
}

#[test]
fn radiobutton_set_current() {
    let radio1 = Radiobutton::new(-1, -1, "Yes", true, None);
    let radio2 = Radiobutton::new(-1, -1, "No", false, Some(&radio1));
    radio2.set_current();
    assert!(radio1.get_current().unwrap() == radio2);
    assert!(radio2.get_current().unwrap() == radio2);
    assert!(radio1.get_current().unwrap() != radio1);
    assert!(radio2.get_current().unwrap() != radio1);
}

#[test]
fn radiobutton_no_current() {
    let radio = Radiobutton::new(-1, -1, "Yes", false, None);
    assert!(radio.get_current().is_none());
}
