use std::cell::Cell;
use std::ffi::CString;
use newt_sys::*;

///
/// A small button with no padding around its label.
///
#[derive(Component)]
pub struct CompactButton {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl CompactButton {
    pub fn new(left: i32, top: i32, text: &str) -> CompactButton {
        let c_str = CString::new(text).unwrap();
        CompactButton {
            co: unsafe {
                let co = newtCompactButton(left, top, c_str.as_ptr());
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }
}
