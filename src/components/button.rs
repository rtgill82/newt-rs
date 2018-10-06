extern crate std;
use std::ffi::CString;

use components::c_component;
use components::Component;
use intern::ffi::newt::button::*;
use intern::ffi::newt::component::newtComponentDestroy;

newt_component!(Button);
pub struct Button {
    co: c_component,
    attached_to_form: bool
}

impl Button {
    pub fn new(left: i32, top: i32, text: &str) -> Button {
        let c_str = CString::new(text).unwrap();
        Button {
            attached_to_form: false,
            co: unsafe { newtButton(left, top, c_str.as_ptr()) }
        }
    }
}
