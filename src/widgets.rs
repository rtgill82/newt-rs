//!
//! newt UI widgets.
//!
use libc::c_int;

use crate::intern::ComponentPtr;
use newt_sys::*;

pub mod form;
#[doc(no_inline)]
pub use self::form::Form;

mod vertical_scrollbar;
pub use self::vertical_scrollbar::VerticalScrollbar;

mod compact_button;
pub use self::compact_button::CompactButton;
mod button;
pub use self::button::Button;
mod checkbox;
pub use self::checkbox::Checkbox;
mod radiobutton;
pub use self::radiobutton::Radiobutton;
mod label;
pub use self::label::Label;
mod listbox;
pub use self::listbox::Listbox;
mod checkbox_tree;
pub use self::checkbox_tree::CheckboxTree;
mod textbox;
pub use self::textbox::Textbox;
mod entry;
pub use self::entry::Entry;
mod scale;
pub use self::scale::Scale;

///
/// Implement shared functions for newt component widgets.
///
pub trait WidgetFns: ComponentPtr {
    ///
    /// Allow the `Widget` to be focused when its parent [`Form`][form] is
    /// run.
    ///
    /// [form]: form/struct.Form.html
    ///
    fn takes_focus(&mut self, value: bool) {
        unsafe { newtComponentTakesFocus(self.as_co(), value as c_int); }
    }

    ///
    /// Get the position of the `Widget`'s top left corner.
    ///
    /// Returns a tuple in the form of `(left, top)`.
    ///
    fn get_position(&self) -> (i32, i32) {
        let mut left: i32 = 0;
        let mut top:  i32 = 0;

        unsafe {
            newtComponentGetPosition(self.as_co(), &mut left, &mut top)
        };
        (left, top)
    }

    ///
    /// Get the `Widget`'s width and height.
    ///
    /// Returns a tuple in the form of `(width, height)`.
    ///
    fn get_size(&self) -> (i32, i32) {
        let mut width:  i32 = 0;
        let mut height: i32 = 0;

        unsafe {
            newtComponentGetSize(self.as_co(), &mut width, &mut height)
        };
        (width, height)
    }
}
