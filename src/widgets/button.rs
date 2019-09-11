use std::cell::Cell;
use std::ffi::CString;

use newt_sys::*;

///
/// A Button widget.
///
#[derive(Component)]
pub struct Button {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Button {
    pub fn new(left: i32, top: i32, text: &str) -> Button {
        let c_str = CString::new(text).unwrap();
        Button {
            co: unsafe {
                Cell::new(newtButton(left, top, c_str.as_ptr()))
            },
            added_to_parent: Cell::new(false)
        }
    }

    #[cfg(feature = "asm")]
    pub(crate) fn new_co(co: newtComponent) -> Button {
        Button {
            co: Cell::new(co),
            added_to_parent: Cell::new(false)
        }
    }
}
