extern crate std;
extern crate newt_sys;

use newt_sys::*;

///
/// A progress bar widget.
///
#[derive(Component)]
pub struct Scale {
    co: newtComponent,
    added_to_parent: bool
}

impl Scale  {
    pub fn new(left: i32, top: i32, width: i32, full_value: i64) -> Scale {
        Scale {
            co: unsafe { newtScale(left, top, width, full_value) },
            added_to_parent: false
        }
    }

    pub fn set(&mut self, amount: u64) {
        unsafe { newtScaleSet(self.co, amount); }
    }

    pub fn set_colors(&mut self, empty: i32, full: i32) {
        unsafe { newtScaleSetColors(self.co, empty, full); }
    }
}
