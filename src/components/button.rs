extern crate std;
extern crate newt_sys;
use std::ffi::CString;

use newt_sys::*;
use components::Component;

newt_component!(Button);
pub struct Button {
    co: newtComponent,
    attached_to_form: bool
}

impl Button {
    pub fn new(left: i32, top: i32, text: &str) -> Button {
        let c_str = CString::new(text).unwrap();
        Button {
            co: unsafe { newtButton(left, top, c_str.as_ptr()) },
            attached_to_form: false
        }
    }
}
