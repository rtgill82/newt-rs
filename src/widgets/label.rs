use std::cell::Cell;
use std::ffi::CString;

use newt_sys::*;

///
/// A simple widget for displaying static text.
///
#[derive(Component)]
pub struct Label {
    co: newtComponent,
    added_to_parent: Cell<bool>
}

impl Label  {
    pub fn new(left: i32, top: i32, text: &str) -> Label {
        let c_text = CString::new(text).unwrap();
        Label {
            co: unsafe { newtLabel(left, top, c_text.as_ptr()) },
            added_to_parent: Cell::new(false)
        }
    }

    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { newtLabelSetText(self.co, c_text.as_ptr()); }
    }

    pub fn set_colors(&self, colorset: i32) {
        unsafe { newtLabelSetColors(self.co, colorset); }
    }
}
