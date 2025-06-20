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
use newt::widgets::{Button,Textbox};
use std::ptr;

use newt::constants::COLORSET_TEXTBOX;
use newt::constants::COLORSET_ACTTEXTBOX;

#[test]
fn textbox_create() {
    let textbox = Textbox::new(-1, -1, 10, 10, 0);
    assert!(textbox.co() != ptr::null_mut());
}

#[test]
fn textbox_create_reflowed() {
    let textbox = Textbox::new_reflowed(-1, -1, "Hello world!",
                                        20, 15, 20, 0);
    assert!(textbox.co() != ptr::null_mut());
}

#[test]
fn textbox_partial_eq_true() {
    let textbox = Textbox::new(-1, -1, 10, 10, 0);
    assert!(textbox == textbox);
}

#[test]
fn textbox_partial_eq_false() {
    let textbox = Textbox::new(-1, -1, 10, 10, 0);
    let button = Button::new(-1, -1, "Ok");
    assert!(textbox != button);
}

#[test]
fn textbox_set_text() {
    let textbox = Textbox::new(-1, -1, 10, 10, 0);
    textbox.set_text("Hello world!");
}

#[test]
fn textbox_set_height() {
    let textbox = Textbox::new(-1, -1, 10, 10, 0);
    textbox.set_height(20);
}

#[test]
fn textbox_get_num_lines() {
    let textbox = Textbox::new(-1, -1, 10, 10, 0);
    textbox.set_text("Hello\nworld!");
    assert!(textbox.get_num_lines() == 2);
}

#[test]
fn textbox_set_colors() {
    let textbox = Textbox::new(-1, -1, 10, 10, 0);
    textbox.set_colors(COLORSET_TEXTBOX, COLORSET_ACTTEXTBOX);
}
