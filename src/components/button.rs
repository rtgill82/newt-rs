extern crate std;
extern crate newt_sys;
use std::ffi::CString;

use newt_sys::*;

#[derive(Component, ComponentFuncs)]
pub struct Button {
    co: newtComponent,
    added_to_parent: bool
}

impl Button {
    pub fn new(left: i32, top: i32, text: &str) -> Button {
        let c_str = CString::new(text).unwrap();
        Button {
            co: unsafe { newtButton(left, top, c_str.as_ptr()) },
            added_to_parent: false
        }
    }

    #[cfg(feature = "asm")]
    pub(crate) fn new_co(co: newtComponent) -> Button {
        Button { co, added_to_parent: false }
    }
}
