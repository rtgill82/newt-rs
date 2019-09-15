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
use newt::widgets::{Form,Scale};
use std::ptr;

use newt::constants::COLORSET_EMPTYSCALE;
use newt::constants::COLORSET_FULLSCALE;

#[test]
fn scale_create() {
    let scale = Scale::new(0, 0, 10, 100);
    assert!(scale.co() != ptr::null_mut());
}

#[test]
fn scale_partial_eq_true() {
    let scale = Scale::new(0, 0, 10, 100);
    assert!(scale == scale);
}

#[test]
fn scale_partial_eq_false() {
    let scale = Scale::new(0, 0, 10, 100);
    let form = Form::new(None, 0);
    assert!(scale != form);
}

#[test]
fn scale_set() {
    let scale = Scale::new(0, 0, 10, 100);
    scale.set(50);
}

#[test]
fn scale_set_colors() {
    let scale = Scale::new(0, 0, 10, 100);
    scale.set_colors(COLORSET_EMPTYSCALE, COLORSET_FULLSCALE);
}
