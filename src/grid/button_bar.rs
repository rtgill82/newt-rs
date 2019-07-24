extern crate newt_sys;
use libc::c_void;
use std::mem::size_of;
use newt_sys::*;

use crate::components::Button;
use crate::intern::asm;

///
/// Creates a row of buttons.
///
#[derive(Grid)]
pub struct ButtonBar {
    grid: newtGrid,
    added_to_parent: bool,

    // Define children as Option for compatibility
    // with macros defined in newt_proc_macros.
    children: Option<Vec<Button>>
}

impl ButtonBar {
    ///
    /// Create a new grid containing a row of buttons. The buttons will
    /// be labeled with the strings provided in `buttons`.
    ///
    /// * `buttons` - A list of strings to use as button labels.
    ///
    pub fn new(buttons: &[&str]) -> ButtonBar {
        unsafe {
            let size = size_of::<newtComponent>() * (buttons.len());
            let buttons_buf = libc::malloc(size) as *mut newtComponent;
            libc::memset(buttons_buf as *mut c_void, 0, size);
            let grid = asm::button_bar_new(buttons, buttons_buf);

            let num_buttons = buttons.len();
            let mut buttons = Vec::new();
            let mut button_co = *buttons_buf.add(num_buttons - 1);
            buttons.push(Button::new_co(button_co));
            for i in (0..num_buttons - 1).rev() {
                button_co = *buttons_buf.add(i);
                buttons.push(Button::new_co(button_co));
            }
            libc::free(buttons_buf as *mut c_void);

            ButtonBar {
                grid,
                added_to_parent: false,
                children: Some(buttons)
            }
        }
    }

    ///
    /// Return the array of buttons contained by the grid.
    ///
    pub fn buttons(&self) -> &[Button] {
        if let Some(buttons) = &self.children {
            return buttons.as_slice();
        }
        unreachable!();
    }
}
