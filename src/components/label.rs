extern crate std;
use std::ffi::CString;

use components::c_component;
use components::Component;
use components::form::ExitReason;
use intern::ffi::newt::label::*;
use intern::ffi::newt::component::newtComponentDestroy;

newt_component!(Label);
pub struct Label {
    co: c_component,
    attached_to_form: bool
}

impl Label  {
    pub fn new(left: i32, top: i32, text: &str) -> Label {
        let c_text = CString::new(text).unwrap();
        Label {
            attached_to_form: false,
            co: unsafe { newtLabel(left, top, c_text.as_ptr()) }
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
