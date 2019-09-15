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
use newt::Component;
use newt::widgets::{Form,Label};
use std::ptr;

use newt::constants::COLORSET_LABEL;

#[test]
fn label_create() {
    let label = Label::new(0, 0, "Ok");
    assert!(label.co() != ptr::null_mut());
}

#[test]
fn label_partial_eq_true() {
    let label = Label::new(0, 0, "Ok");
    assert!(label == label);
}

#[test]
fn label_partial_eq_false() {
    let label = Label::new(0, 0, "Ok");
    let form = Form::new(None, 0);
    assert!(label != form);
}

#[test]
fn label_set_text() {
    let label = Label::new(0, 0, "Ok");
    label.set_text("Not Ok");
}

#[test]
fn label_set_colors() {
    let label = Label::new(0, 0, "Ok");
    label.set_colors(COLORSET_LABEL);
}
