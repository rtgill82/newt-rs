extern crate std;
extern crate newt_sys;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use crate::ptr;

use newt_sys::*;
use crate::components::Component;
use crate::constants::FlagsSense;
use crate::intern::funcs::newt_entry_set_filter;

#[derive(Component)]
pub struct Entry {
    co: newtComponent,
    attached_to_form: bool
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
            attached_to_form: false
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

pub struct EntryFilter<'a, FN: 'a, T: 'a>
where FN: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    func: FN,
    entries: Vec<(&'a Entry, Option<T>)>
}

impl<'a, FN: 'a, T: 'a> EntryFilter<'a, FN, T>
where FN: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    pub fn new(entry: &'a Entry, function: FN, data: Option<T>)
      -> Box<EntryFilter<'a, FN, T>> {
        let co: newtComponent = entry.co();
        let filter = Box::new(EntryFilter {
            func: function,
            entries: vec![(entry, data)]
        });
        newt_entry_set_filter(co, filter.as_ref());
        return filter;
    }

    pub fn add_entry(&mut self, entry: &'a Entry, data: Option<T>) {
        let co: newtComponent = entry.co();
        self.entries.push((entry, data));
        newt_entry_set_filter(co, self);
    }

    pub(crate) fn call(&mut self, co: newtComponent, ch: char, cursor: i32)
      -> char {
        for (entry, data) in self.entries.iter() {
            if entry.co() == co {
                return (self.func)(*entry, data.as_ref(), ch, cursor);
            }
        }
        return '\0';
    }
}
