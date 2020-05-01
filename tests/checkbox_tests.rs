//
// Copyright (C) 2019 Robert Gill <locke@sdf.org>
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
use newt::widgets::{Checkbox,Form};
use std::ptr;

use newt::constants::FlagsSense;

#[test]
fn checkbox_create() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    assert!(checkbox.co() != ptr::null_mut());
}

#[test]
fn checkbox_partial_eq_true() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    assert!(checkbox == checkbox);
}

#[test]
fn checkbox_partial_eq_false() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    let form = Form::new(None, 0);
    assert!(checkbox != form);
}

#[test]
fn checkbox_get_value() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    assert!(checkbox.get_value() == ' ');
}

#[test]
fn checkbox_set_value() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    assert!(checkbox.get_value() == ' ');
    checkbox.set_value('X');
    assert!(checkbox.get_value() == 'X');
}

#[test]
fn checkbox_set_flags() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    checkbox.set_flags(0, FlagsSense::Reset);
}
