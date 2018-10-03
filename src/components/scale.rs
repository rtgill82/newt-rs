extern crate std;
use std::os::raw::{c_int, c_longlong, c_ulonglong};

use components::c_component;
use components::Component;

newt_component!(Scale);
pub struct Scale {
    co: c_component
}

impl Scale  {
    pub fn new(left: i32, top: i32, width: i32, full_value: i64) -> Scale {
        #[link(name="newt")]
        extern "C" {
            fn newtScale(left: c_int, top: c_int, width: c_int,
                         fullValue: c_longlong) -> c_component;
        }

        Scale {
            co: unsafe { newtScale(left, top, width, full_value) }
        }
    }

    pub fn set(&self, amount: u64) {
        #[link(name="newt")]
        extern "C" {
            fn newtScaleSet(co: c_component, amount: c_ulonglong);
        }

        unsafe { newtScaleSet(self.co, amount); }
    }

    pub fn set_colors(&self, empty: i32, full: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtScaleSetColors(co: c_component, empty: c_int, full: c_int);
        }

        unsafe { newtScaleSetColors(self.co, empty, full); }
    }
}
