extern crate std;
extern crate newt_sys;
use std::ffi::CString;

use newt_sys::*;

#[derive(Component)]
pub struct Label {
    co: newtComponent,
    added_to_parent: bool
}

impl Label  {
    pub fn new(left: i32, top: i32, text: &str) -> Label {
        let c_text = CString::new(text).unwrap();
        Label {
            co: unsafe { newtLabel(left, top, c_text.as_ptr()) },
            added_to_parent: false
        }
    }

    pub fn set_text(&mut self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { newtLabelSetText(self.co, c_text.as_ptr()); }
    }

    pub fn set_colors(&mut self, colorset: i32) {
        unsafe { newtLabelSetColors(self.co, colorset); }
    }
}
