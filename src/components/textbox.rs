extern crate std;
use std::ffi::CString;
use std::os::raw::c_int;
use std::os::raw::c_char;

use components::NewtComponent;
use components::NewtComponentPtr;

newt_component!(Textbox);
pub struct Textbox {
    co: NewtComponentPtr
}

impl Textbox {
    pub fn new(left: i32, top: i32, width: i32, height: i32, flags: i32)
            -> Textbox {
        #[link(name="newt")]
        extern "C" {
           fn  newtTextbox(left: c_int, top: c_int, width: c_int,
                           height: c_int, flags: c_int) -> NewtComponentPtr;
        }

        Textbox {
            co: unsafe { newtTextbox(left, top, width, height, flags) }
        }
    }

    pub fn new_reflowed(left: i32, top: i32, text: &str, width: i32,
                        flex_down: i32, flex_up: i32, flags: i32) -> Textbox {
        #[link(name="newt")]
        extern "C" {
           fn  newtTextboxReflowed(left: c_int, top: c_int,
                                   text: *const c_char, width: c_int,
                                   flexDown: c_int, flexUp: c_int,
                                   flags: c_int) -> NewtComponentPtr;
        }

        let c_text = CString::new(text).unwrap();
        Textbox {
            co: unsafe {
                newtTextboxReflowed(left, top, c_text.as_ptr(), width,
                                    flex_down, flex_up, flags)
            }
        }
    }

    pub fn set_text(&self, text: &str) {
        #[link(name="newt")]
        extern "C" {
            fn newtTextboxSetText(co: NewtComponentPtr, text: *const c_char);
        }

        let c_text = CString::new(text).unwrap();
        unsafe { newtTextboxSetText(self.co, c_text.as_ptr()); }
    }

    pub fn set_height(&self, height: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtTextboxSetHeight(co: NewtComponentPtr, height: c_int);
        }

        unsafe { newtTextboxSetHeight(self.co, height); }
    }

    pub fn get_num_lines(&self) -> i32 {
        #[link(name="newt")]
        extern "C" {
            fn newtTextboxGetNumLines(co: NewtComponentPtr) -> c_int;
        }

        unsafe { newtTextboxGetNumLines(self.co) }
    }

    pub fn set_colors(&self, normal: i32, active: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtTextboxSetColors(co: NewtComponentPtr, normal: c_int,
                                    active: c_int);
        }

        unsafe { newtTextboxSetColors(self.co, normal, active); }
    }
}
