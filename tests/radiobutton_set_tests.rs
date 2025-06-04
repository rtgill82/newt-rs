//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
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
use newt::widgets::RadiobuttonSet;

#[test]
fn radiobutton_set_create() {
    let _set = RadiobuttonSet::new();
}

#[test]
fn radiobutton_set_add_radiobutton() {
    let mut set = RadiobuttonSet::new();
    let n = set.add_radiobutton(-1, -1, "radio1");
    assert!(n == 1);
    let n = set.add_radiobutton(-1, -1, "radio2");
    assert!(n == 2);
}

#[test]
fn radiobutton_set_get_current() {
    let mut set = RadiobuttonSet::new();
    set.add_radiobutton(-1, -1, "radio1");
    assert!(set.get_current() == 0);
}

#[test]
fn radiobutton_set_set_current() {
    let mut set = RadiobuttonSet::new();
    set.add_radiobutton(-1, -1, "radio1");
    set.add_radiobutton(-1, -1, "radio2");
    set.add_radiobutton(-1, -1, "radio3");
    assert!(set.get_current() == 0);
    set.set_current(2);
    assert!(set.get_current() == 2);
}

#[test]
#[should_panic(expected="index out of bounds: the len is 0 but the index is 0")]
fn radiobutton_set_get_current_index_error() {
    let set = RadiobuttonSet::new();
    set.get_current();
}

#[test]
#[should_panic(expected="index out of bounds: the len is 0 but the index is 1")]
fn radiobutton_set_set_current_index_error() {
    let set = RadiobuttonSet::new();
    set.set_current(1);
}
