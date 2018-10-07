extern crate std;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::Deref;
use std::os::raw::{c_int,c_void};

use components::c_component;
use components::form::ExitReason;
use intern::ffi::newt::component::*;

pub struct Data<'a, T: 'a>(pub &'a T);

impl<'a, T: 'a> ::components::data::Data for Data<'a, T> {
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

pub trait Component {
    fn co(&self) -> c_component;
    fn attach_to_form(&mut self);
    fn attached_to_form(&self) -> bool;

    fn takes_focus(&mut self, value: bool) {
        unsafe { newtComponentTakesFocus(self.co(), value as c_int); }
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
        if let &ExitReason::Component(ref component) = other {
            return self.co() == component.co()
        }
        return false;
    }
}

impl<Rhs: Component> PartialEq<Rhs> for Box<dyn Component> {
    fn eq(&self, other: &Rhs) -> bool {
        self.co() == other.co()
    }
}
