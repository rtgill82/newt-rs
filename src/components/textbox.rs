extern crate std;
use std::ffi::CString;

use components::c_component;
use components::Component;
use intern::ffi::newt::textbox::*;

newt_component!(Textbox);
pub struct Textbox {
    co: c_component,
    attached_to_form: bool
}

impl Textbox {
    pub fn new(left: i32, top: i32, width: i32, height: i32, flags: i32)
            -> Textbox {
        Textbox {
            co: unsafe { newtTextbox(left, top, width, height, flags) },
            attached_to_form: false
        }
    }

    pub fn new_reflowed(left: i32, top: i32, text: &str, width: i32,
                        flex_down: i32, flex_up: i32, flags: i32) -> Textbox {
        let c_text = CString::new(text).unwrap();
        Textbox {
            co: unsafe {
                newtTextboxReflowed(left, top, c_text.as_ptr(), width,
                                    flex_down, flex_up, flags)
            },
            attached_to_form: false
        }
    }

    pub fn set_text(&mut self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { newtTextboxSetText(self.co, c_text.as_ptr()); }
    }

    pub fn set_height(&mut self, height: i32) {
        unsafe { newtTextboxSetHeight(self.co, height); }
    }

    pub fn get_num_lines(&self) -> i32 {
        unsafe { newtTextboxGetNumLines(self.co) }
    }

    pub fn set_colors(&mut self, normal: i32, active: i32) {
        unsafe { newtTextboxSetColors(self.co, normal, active); }
    }
}
