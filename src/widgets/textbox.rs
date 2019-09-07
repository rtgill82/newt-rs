use std::cell::Cell;
use std::ffi::CString;
use std::os::raw::c_char;

use newt_sys::*;

///
/// A multi-line Textbox widget.
///
#[derive(Component)]
pub struct Textbox {
    co: newtComponent,
    added_to_parent: Cell<bool>
}

impl Textbox {
    pub fn new(left: i32, top: i32, width: i32, height: i32, flags: i32)
            -> Textbox {
        Textbox {
            co: unsafe { newtTextbox(left, top, width, height, flags) },
            added_to_parent: Cell::new(false)
        }
    }

    pub fn new_reflowed(left: i32, top: i32, text: &str, width: i32,
                        flex_down: i32, flex_up: i32, flags: i32) -> Textbox {
        let c_text = CString::new(text).unwrap();
        Textbox {
            co: unsafe {
                newtTextboxReflowed(left, top, c_text.as_ptr() as *mut c_char,
                                    width, flex_down, flex_up, flags)
            },
            added_to_parent: Cell::new(false)
        }
    }

    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { newtTextboxSetText(self.co, c_text.as_ptr()); }
    }

    pub fn set_height(&self, height: i32) {
        unsafe { newtTextboxSetHeight(self.co, height); }
    }

    pub fn get_num_lines(&self) -> i32 {
        unsafe { newtTextboxGetNumLines(self.co) }
    }

    pub fn set_colors(&self, normal: i32, active: i32) {
        unsafe { newtTextboxSetColors(self.co, normal, active); }
    }
}
