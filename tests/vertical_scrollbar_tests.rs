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
use newt::widgets::{Form,VerticalScrollbar};
use std::ptr;

#[test]
fn vertical_scrollbar_create() {
    let scrollbar = VerticalScrollbar::new(0, 0, 10, 0, 0);
    assert!(scrollbar.co() != ptr::null_mut());
}

#[test]
fn vertical_scrollbar_partial_eq_false() {
    let scrollbar = VerticalScrollbar::new(0, 0, 10, 0, 0);
    let form = Form::new(Some(&scrollbar), 0);
    assert!(scrollbar == scrollbar);
    assert!(scrollbar != form);
}
