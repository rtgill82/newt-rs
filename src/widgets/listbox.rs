//
// Copyright (C) 2019 Robert Gill <rtgill82@gmail.com>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char,c_uint,c_void};
use std::ptr;

use newt_sys::*;
use crate::component::Component;
use crate::constants::FlagsSense;
use crate::private::data::Data;

///
/// A widget for displaying a list of selectable items.
///
#[derive(Component)]
pub struct Listbox<D: Data = isize> {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>,
    data: PhantomData<D>
}

impl<D: Data> Listbox<D> {
    ///
    /// Create a new `Listbox`.
    ///
    /// * `left` - The left-most position of the `Listbox`.
    /// * `top` - The top-most position of the `Listbox`.
    /// * `height` - The height of the `Listbox`.
    /// * `flags` - [Flags][listbox] modifying the behavior of the `Listbox`.
    ///             See also [generalized flags][flags].
    ///
    /// [listbox]: crate::constants::listbox
    /// [flags]: crate::constants::flags
    ///
    pub fn new(left: i32, top: i32, height: i32, flags: i32)
      -> Listbox<D> {
        Listbox {
            co: unsafe {
                let co = newtListbox(left, top, height, flags);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false),
            data: PhantomData
        }
    }

    ///
    /// Set the width of the `Listbox`.
    ///
    /// * `width` - The new width of the `Listbox`.
    ///
    pub fn set_width(&self, width: i32) {
        unsafe { newtListboxSetWidth(self.co(), width); }
    }

    ///
    /// Get the number of items in the `Listbox`.
    ///
    pub fn item_count(&self) -> i32 {
        unsafe { newtListboxItemCount(self.co()) }
    }

    ///
    /// Add a new item to the `Listbox`.
    ///
    /// * `text` - The displayed text of the item.
    /// * `data` - The user `Data` associated with the item.
    ///
    pub fn append_entry(&self, text: &str, data: D) -> Result<(), ()> {
        let c_str = CString::new(text).unwrap();
        let rv = unsafe {
            newtListboxAppendEntry(
                self.co(),
                c_str.as_ptr(),
                data.newt_to_ptr()
            )
        };
        if rv == 0 { Ok(()) } else { Err(()) }
    }

    ///
    /// Add a new item to the `Listbox` before the item associated with
    /// user `Data` `key`.
    ///
    /// * `text` - The displayed text of the item.
    /// * `data` - The user `Data` associated with the item.
    /// * `key` - The user `Data` of the item this item is to be inserted
    ///           before.
    ///
    pub fn insert_entry(&self, text: &str, data: D, key: D)
          -> Result<(), ()> {
        let c_str = CString::new(text).unwrap();
        let rv = unsafe {
            newtListboxInsertEntry(self.co(), c_str.as_ptr(),
                                   data.newt_to_ptr(),
                                   key.newt_to_ptr() as *mut c_void)
        };
        if rv == 0 { Ok(()) } else { Err(()) }
    }

    ///
    /// Get the user `Data` of the currently selected item in the `Listbox`.
    ///
    pub fn get_current(&self) -> Option<D> {
        let c_data = unsafe { newtListboxGetCurrent(self.co()) };
        if c_data.is_null() { return None; }
        Some(D::newt_from_ptr(c_data))
    }

    ///
    /// Set the currently selected item in the `Listbox` by index number.
    ///
    /// * `num` - The index number of the item to be set as the currently
    ///           selected item.
    ///
    pub fn set_current(&self, num: i32) {
        unsafe { newtListboxSetCurrent(self.co(), num); }
    }

    ///
    /// Set the currently selected item in the `Listbox` by associated
    /// user `Data`.
    ///
    /// * `key` - The user `Data` associated with the item to be set as the
    ///           currently selected item.
    ///
    pub fn set_current_by_key(&self, key: D) {
        unsafe {
            newtListboxSetCurrentByKey(
                self.co(),
                key.newt_to_ptr() as *mut c_void
            );
        }
    }

    ///
    /// Get an item in the `Listbox` by index number.
    ///
    /// * `num` - The index number of the item.
    ///
    /// `Returns` a tuple (`text`, `data`) containing the items display text
    /// and the user `Data` associated with it.
    ///
    pub fn get_entry(&self, num: i32) -> (&str, D) {
        let mut c_str: *mut c_char = ptr::null_mut();
        let mut c_data: *mut c_void = ptr::null_mut();

        unsafe {
            newtListboxGetEntry(self.co(), num, &mut c_str, &mut c_data);
        }
        let c_str = unsafe { CStr::from_ptr(c_str) };
        (c_str.to_str().unwrap(), D::newt_from_ptr(c_data))
    }

    ///
    /// Set an item's display text in the `Listbox` by index number.
    ///
    /// * `num` - The index number of the item to be modified.
    /// * `text` - The new display text of the item.
    ///
    pub fn set_entry(&self, num: i32, text: &str) {
        let c_str = CString::new(text).unwrap();
        unsafe { newtListboxSetEntry(self.co(), num, c_str.as_ptr()); }
    }

    ///
    /// Set an item's associated user `Data` in the `Listbox` by index number.
    ///
    /// * `num` - The index number of the item to be modified.
    /// * `data` - The new user `Data` to be associated with the item.
    ///
    pub fn set_data(&self, num: i32, data: D) {
        unsafe {
            newtListboxSetData(
                self.co(),
                num,
                data.newt_to_ptr() as *mut c_void
            );
        }
    }

    ///
    /// Delete an item from the `Listbox` by associated user `Data`.
    ///
    /// * `data` - The user `Data` associated with the item to be deleted.
    ///
    /// `Returns` the index number of the deleted item.
    ///
    pub fn delete_entry(&self, data: D) -> i32 {
        unsafe {
            newtListboxDeleteEntry(
                self.co(),
                data.newt_to_ptr() as *mut c_void
            )
        }
    }

    ///
    /// Delete all items in a `Listbox`.
    ///
    pub fn clear(&self) {
        unsafe { newtListboxClear(self.co()); }
    }

    ///
    /// Get the current selections in the `Listbox`.
    ///
    /// `Returns` an array of the user `Data` associated with the current
    /// selections.
    ///
    pub fn get_selection(&self) -> Box<[D]> {
        let mut numitems: i32 = 0;
        let ptr = unsafe { newtListboxGetSelection(self.co(), &mut numitems) };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    ///
    /// Modify an item's current selection status.
    ///
    /// * `key` - The user `Data` associated with the item.
    /// * `sense` - The sense in which the selection should be modified
    ///             (`Set`, `Reset`, or `Toggle`).
    ///
    pub fn select_item(&self, key: D, sense: FlagsSense) {
        unsafe {
            newtListboxSelectItem(
                self.co(),
                key.newt_to_ptr(),
                sense as c_uint
            );
        }
    }

    ///
    /// Modify multiple items' current selection status.
    ///
    /// * `keys` - An array of user `Data` associated with the items.
    /// * `sense` - The sense in which the selection should be modified
    ///             (`Set`, `Reset`, or `Toggle`).
    ///
    pub fn select_items(&self, keys: &[D], sense: FlagsSense) {
        unsafe {
            for key in keys {
                newtListboxSelectItem(
                    self.co(),
                    key.newt_to_ptr(),
                    sense as c_uint
                );
            }
        }
    }

    ///
    /// Clear all selections in the `Listbox`.
    ///
    pub fn clear_selection(&self) {
        unsafe { newtListboxClearSelection(self.co()) };
    }
}
