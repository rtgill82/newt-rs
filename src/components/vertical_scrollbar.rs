extern crate std;
extern crate newt_sys;

use newt_sys::*;
use crate::ptr;

#[derive(Component)]
pub struct VerticalScrollbar {
    co: newtComponent,
    added_to_parent: bool
}

impl VerticalScrollbar  {
    pub fn new(_left: i32, _top: i32, _height: i32, _normal_colorset: i32,
               _thumb_colorset: i32) -> VerticalScrollbar {
        VerticalScrollbar {
            co: ptr::null_mut(),
            added_to_parent: false
        }
    }
}
