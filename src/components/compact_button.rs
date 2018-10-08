extern crate std;
use std::ffi::CString;

use components::c_component;
use components::Component;
use intern::ffi::newt::compact_button::*;

newt_component!(CompactButton);
pub struct CompactButton {
    co: c_component,
    attached_to_form: bool
}

impl CompactButton {
    pub fn new(left: i32, top: i32, text: &str) -> CompactButton {
        let c_str = CString::new(text).unwrap();
        CompactButton {
            co: unsafe { newtCompactButton(left, top, c_str.as_ptr()) },
            attached_to_form: false
        }
    }
}
