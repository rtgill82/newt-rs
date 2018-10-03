extern crate std;
use std::os::raw::{c_int, c_longlong, c_ulonglong};

use components::Component;
use components::NewtComponentPtr;

newt_component!(Scale);
pub struct Scale {
    co: NewtComponentPtr
}

impl Scale  {
    pub fn new(left: i32, top: i32, width: i32, full_value: i64) -> Scale {
        #[link(name="newt")]
        extern "C" {
            fn newtScale(left: c_int, top: c_int, width: c_int,
                         fullValue: c_longlong) -> NewtComponentPtr;
        }

        Scale {
            co: unsafe { newtScale(left, top, width, full_value) }
        }
    }

    pub fn set(&self, amount: u64) {
        #[link(name="newt")]
        extern "C" {
            fn newtScaleSet(co: NewtComponentPtr, amount: c_ulonglong);
        }

        unsafe { newtScaleSet(self.co, amount); }
    }

    pub fn set_colors(&self, empty: i32, full: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtScaleSetColors(co: NewtComponentPtr, empty: c_int,
                                  full: c_int);
        }

        unsafe { newtScaleSetColors(self.co, empty, full); }
    }
}
