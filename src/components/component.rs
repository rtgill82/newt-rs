extern crate std;
extern crate newt_sys;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Deref;
use std::os::raw::{c_int,c_void};

use newt_sys::*;
use crate::components::form::ExitReason;
use crate::intern::{Child,GridElementType};

pub struct Data<'a, T: 'a>(pub &'a T);

impl<'a, T: 'a> crate::intern::data::Data for Data<'a, T> {
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

pub trait Component: Child + GridElementType {
    fn co(&self) -> newtComponent;
    fn grid(&self) -> newtGrid;
}

///
/// Implement shared functions for newt components.
///
pub trait ComponentFuncs: Component {
    ///
    /// Allow the `Component` to be focused when it's [`Form`][form] is run.
    ///
    /// [form]: ../components/form/struct.Form.html
    ///
    fn takes_focus(&mut self, value: bool) {
        unsafe { newtComponentTakesFocus(self.co(), value as c_int); }
    }

    ///
    /// Get the position of the `Component`'s top left corner.
    ///
    /// Returns a tuple in the form of (left, top).
    ///
    fn get_position(&self) -> (i32, i32) {
        let mut left: i32 = 0;
        let mut top:  i32 = 0;

        unsafe { newtComponentGetPosition(self.co(), &mut left, &mut top) };
        (left, top)
    }

    ///
    /// Get the `Component`'s width and height.
    ///
    /// Returns a tuple in the form of (width, height).
    ///
    fn get_size(&self) -> (i32, i32) {
        let mut width:  i32 = 0;
        let mut height: i32 = 0;

        unsafe { newtComponentGetSize(self.co(), &mut width, &mut height) };
        (width, height)
    }
}

impl Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Component {{ {:p} }}", self.co())
    }
}

impl PartialEq for Component {
    fn eq(&self, other: &dyn Component) -> bool {
        self.co() == other.co()
    }
}

impl PartialEq<ExitReason> for Component {
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
