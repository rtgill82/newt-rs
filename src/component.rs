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

//!
//! Trait implemented by [widget][widget] and [`Grid`][grid] types.
//!
//! [grid]: crate::grid
//! [widget]: crate::widgets
//!
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Deref;
use std::os::raw::c_void;

use newt_sys::*;
use crate::form::ExitReason;
use crate::widgets::WidgetFns;

use crate::asm::*;
use crate::private::traits::*;
use crate::private;

///
/// A wrapper for passing complex data to [CheckboxTree][checkbox_tree] and
/// [Listbox][listbox] widgets.
///
/// [checkbox_tree]: crate::widgets::CheckboxTree
/// [listbox]: crate::widgets::Listbox
///
pub struct Data<'a, T: 'a>(pub &'a T);

impl<'a, T: 'a> private::data::Data for Data<'a, T> {
    fn newt_to_ptr(&self) -> *const c_void {
        self.0 as *const _ as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        let data = unsafe { &*(ptr as *const T) };
        Data(data)
    }
}

impl<'a, T> Deref for Data<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

///
/// Trait implemented by `Widget` types and `Grid` types.
///
pub trait Component: AsComponent + AsGrid + Child + GridElementType + Nullify
                                 + WidgetFns
{
    /// Return `newtComponent` pointer.
    fn co(&self) -> newtComponent;
}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Component {{ {:p} }}", self.co())
    }
}

impl PartialEq for dyn Component {
    fn eq(&self, other: &dyn Component) -> bool {
        self.co() == other.co()
    }
}

impl PartialEq<ExitReason> for dyn Component {
    fn eq(&self, other: &ExitReason) -> bool {
        if let ExitReason::Component(ref component) = other {
            return self.co() == component.co()
        }
        false
    }
}

impl<Rhs: Component> PartialEq<Rhs> for Box<dyn Component> {
    fn eq(&self, other: &Rhs) -> bool {
        self.co() == other.co()
    }
}
