extern crate std;

use components::c_component;
use components::Component;
use intern::ffi::newt::scale::*;
use intern::ffi::newt::component::newtComponentDestroy;

newt_component!(Scale);
pub struct Scale {
    co: c_component,
    attached_to_form: bool
}

impl Scale  {
    pub fn new(left: i32, top: i32, width: i32, full_value: i64) -> Scale {
        Scale {
            attached_to_form: false,
            co: unsafe { newtScale(left, top, width, full_value) }
        }
    }

    pub fn set(&mut self, amount: u64) {
        unsafe { newtScaleSet(self.co, amount); }
    }

    pub fn set_colors(&mut self, empty: i32, full: i32) {
        unsafe { newtScaleSetColors(self.co, empty, full); }
    }
}
