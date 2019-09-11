use std::cell::Cell;

use newt_sys::*;
use crate::component::Component;

///
/// A progress bar widget.
///
#[derive(Component)]
pub struct Scale {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Scale {
    pub fn new(left: i32, top: i32, width: i32, full_value: i64) -> Scale {
        Scale {
            co: unsafe {
                let co = newtScale(left, top, width, full_value);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    pub fn set(&self, amount: u64) {
        unsafe { newtScaleSet(self.co(), amount); }
    }

    pub fn set_colors(&self, empty: i32, full: i32) {
        unsafe { newtScaleSetColors(self.co(), empty, full); }
    }
}
