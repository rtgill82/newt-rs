extern crate std;
extern crate newt_sys;
use std::ffi::CString;

use newt_sys::*;

#[derive(Component)]
pub struct CompactButton {
    co: newtComponent,
    added_to_form: bool
}

impl CompactButton {
    pub fn new(left: i32, top: i32, text: &str) -> CompactButton {
        let c_str = CString::new(text).unwrap();
        CompactButton {
            co: unsafe { newtCompactButton(left, top, c_str.as_ptr()) },
            added_to_form: false
        }
    }
}
