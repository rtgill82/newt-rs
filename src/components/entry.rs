extern crate std;
extern crate newt_sys;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use crate::ptr;

use newt_sys::*;
use crate::constants::FlagsSense;

///
/// A field for reading text input from the user.
///
/// An [EntryFilter][entry_filter] can be use to filter characters as the user
/// enters them into the `Entry`.
///
/// [entry_filter]: ../callbacks/struct.EntryFilter.html
///
/// ## Example
///
/// ```rust no_run
/// extern crate newt;
/// use newt::callbacks::EntryFilter;
/// use newt::prelude::*;
///
/// pub fn main() {
///     newt::init().unwrap();
///     newt::cls();
///     newt::centered_window(22, 5, None).unwrap();
///
///     // last character entered
///     let mut g_ch: char = '\0';
///     // last cursor position
///     let mut g_cursor: i32 = 0;
///     // user data from last Entry modified
///     let mut g_data: i32 = 0;
///
///     // Create closure to be used to filter the Entry field.
///     let mut f = |_e: &Entry, data: Option<&i32>, ch: char, cursor: i32| {
///         g_data = *data.unwrap(); // set user data
///         g_ch = ch;               // set character entered
///         g_cursor = cursor;       // set cursor position
///
///         // The returned character gets added to the entry.
///         // If for example you want to fill an entry with asterisks despite
///         // what the user entered, then return '*' here.
///         return ch; // Return the entered character.
///     };
///
///     let mut form = Form::new(None, 0);
///     let mut l1 = Label::new(1, 1, "Entry 1:");
///     let mut l2 = Label::new(1, 2, "Entry 2:");
///     let mut e1 = Entry::new(10, 1, None, 10, 0);
///     let mut e2 = Entry::new(10, 2, None, 10, 0);
///     let mut ok = CompactButton::new(7, 4, "Ok");
///
///     form.add_components(&mut [&mut l1, &mut l2, &mut e1, &mut e2, &mut ok])
///         .unwrap();
///
///     // Filter the first Entry, passing user data `5`.
///     let mut filter = EntryFilter::new(&e1, &mut f, Some(5));
///     // Filter the second Entry, passing user data `10`.
///     filter.add_entry(&e2, Some(10));
///
///     form.run().unwrap();
///     newt::finished();
///
///     println!("Entry 1: {}", e1.get_text());
///     println!("Entry 2: {}", e2.get_text());
///
///     // Display the last values set by the filter.
///     println!("ch = {}", g_ch);
///     println!("cursor = {}", g_cursor);
///     println!("data = {}", g_data);
/// }
/// ```
///
#[derive(Component)]
pub struct Entry {
    co: newtComponent,
    added_to_parent: bool
}

impl Entry  {
    pub fn new(left: i32, top: i32, initial_value: Option<&str>, width: i32,
               flags: i32) -> Entry {
        let c_str: CString;
        let ptr = match initial_value {
            Some(text) => {
                c_str = CString::new(text).unwrap();
                c_str.as_ptr()
            },
            None => ptr::null()
        };

        Entry {
            co: unsafe {
                newtEntry(left, top, ptr, width, ptr::null_mut(), flags)
            },
            added_to_parent: false
        }
    }

    pub fn get_text(&self) -> String {
        unsafe { CStr::from_ptr(newtEntryGetValue(self.co)) }
            .to_string_lossy()
            .into_owned()
    }

    pub fn set_text(&mut self, text: &str, cursor_at_end: bool) {
        let c_str = CString::new(text).unwrap();
        unsafe {
            newtEntrySet(self.co, c_str.as_ptr(), cursor_at_end as c_int);
        }
    }

    pub fn set_flags(&mut self, flags: i32, sense: FlagsSense) {
        unsafe { newtEntrySetFlags(self.co, flags, sense as u32); }
    }

    pub fn set_colors(&mut self, normal: i32, disabled: i32) {
        unsafe { newtEntrySetColors(self.co, normal, disabled); }
    }

    pub fn get_cursor_position(&self) -> i32 {
        unsafe { newtEntryGetCursorPosition(self.co) }
    }

    pub fn set_cursor_position(&mut self, position: i32) {
        unsafe { newtEntrySetCursorPosition(self.co, position) }
    }
}
