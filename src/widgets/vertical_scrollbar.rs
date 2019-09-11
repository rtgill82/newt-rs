use std::cell::Cell;
use newt_sys::*;

///
/// A VerticalScrollbar widget.
///
#[derive(Component)]
pub struct VerticalScrollbar {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl VerticalScrollbar {
    pub fn new(left: i32, top: i32, height: i32, normal_colorset: i32,
               thumb_colorset: i32) -> VerticalScrollbar {
        VerticalScrollbar {
            co: unsafe {
                let co = newtVerticalScrollbar (left, top, height,
                                                normal_colorset,
                                                thumb_colorset);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }
}
