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
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};

use newt_sys::*;
use crate::component::Component;
use crate::intern::data::Data;
use crate::intern::funcs::char_slice_to_cstring;
use crate::constants;

///
/// [Checkboxes][checkbox] arranged in a collapsible tree.
///
/// [checkbox]: crate::widgets::Checkbox
///
/// ## Example
/// ```rust no_run
/// extern crate newt;
/// use newt::prelude::*;
///
/// pub fn main() {
///     newt::init().unwrap();
///     newt::cls();
///     newt::centered_window(20, 9, Some("Options")).unwrap();
///
///     let tree: CheckboxTree =
///         CheckboxTree::new(0, 0, 7, Some(&[' ', 'A', 'B']), 0);
///     let ok = CompactButton::new(7, 8, "Ok");
///
///     // A root checkbox, no children possible.
///     tree.add_item("Option 0", 0, 0, None);
///
///     // Create a new root tree, with and index of `0`.
///     tree.add_item("Tree 1", 1, 0, Some(&[0]));
///     // Add children to root tree `0`.
///     tree.add_item("Option 1", 2, 0, Some(&[0, ARG_APPEND]));
///     tree.add_item("Option 2", 3, 0, Some(&[0, ARG_APPEND]));
///
///     // Create a new root tree, 'Tree 2', with an index of `1`.
///     tree.add_item("Tree 2", 4, 0, Some(&[1]));
///
///     // Add a second tree under root tree `1`.
///     tree.add_item("Tree 3", 5, 0, Some(&[1, ARG_APPEND]));
///
///     // Append a checkbox under 'Tree 3'. `1` refers to the root
///     // tree, 'Tree 2'. `0` refers to the index of the first item under
///     // 'Tree 2', 'Tree 3'.
///     tree.add_item("Option 6", 9, 0, Some(&[1, 0, ARG_APPEND]));
///
///     // Append the last few checkboxes under 'Tree 2'.
///     tree.add_item("Option 3", 6, 0, Some(&[1, ARG_APPEND]));
///     tree.add_item("Option 4", 7, 0, Some(&[1, ARG_APPEND]));
///     tree.add_item("Option 5", 8, 0, Some(&[1, ARG_APPEND]));
///
///     let mut form = Form::new(None, 0);
///     form.add_components(&[&tree, &ok]).unwrap();
///     form.run().unwrap();
///     newt::finished();
///
///     let selection = tree.get_selection();
///     println!("selection: {:?}", selection);
///     for i in selection.iter() {
///         println!("{} is set to {}", i, tree.get_entry_value(*i));
///     }
/// }
/// ```
///
#[derive(Component)]
pub struct CheckboxTree<D: Data = isize> {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>,
    data: PhantomData<D>
}

impl<D: Data> CheckboxTree<D> {
    ///
    /// Create a new `CheckboxTree`.
    ///
    /// * `left` - The left-most position of the `CheckboxTree`.
    /// * `top` - The top-most position of the `CheckboxTree`.
    /// * `height` - The height of the `CheckboxTree`.
    /// * `sequence` - The optional sequence of values possible in the
    ///                checkboxes. See [Checkbox][checkbox].
    /// * `flags` - [Flags][checkboxtree] modifying the `CheckboxTree`
    ///             behavior. See also [general flags][flags].
    ///
    /// [checkbox]: crate::widgets::Checkbox::new
    /// [checkboxtree]: crate::constants::checkboxtree
    /// [flags]: crate::constants::flags
    ///
    pub fn new(left: i32, top: i32, height: i32, sequence: Option<&[char]>,
               flags: i32) -> CheckboxTree<D>
    {
        let component: newtComponent = match sequence {
            Some(seq) => {
                let c_seq = char_slice_to_cstring(&seq);
                unsafe {
                    newtCheckboxTreeMulti(left, top, height,
                                          c_seq.as_ptr() as *mut c_char,
                                          flags)
                }
            },

            None => unsafe { newtCheckboxTree(left, top, height, flags) }
        };

        CheckboxTree {
            co: Cell::new(component),
            added_to_parent: Cell::new(false),
            data: PhantomData
        }
    }

    ///
    /// Set the width of the `CheckboxTree`.
    ///
    /// * `width` - The width to set the `CheckboxTree`.
    ///
    pub fn set_width(&self, width: i32) {
        unsafe { newtCheckboxTreeSetWidth(self.co(), width); }
    }

    ///
    /// Add an additional checkbox to the `CheckboxTree`.
    ///
    /// A checkbox is identified with an optional index. If no index is
    /// provided it's a root checkbox with no children. One index allows a
    /// checkbox to be the root of a tree. Children are added to a tree
    /// using the index **[_root_index_, `ARG_APPEND`]** where _root_index_
    /// is the index _id_ of the parent node. See [example][example].
    ///
    /// * `text` - The label to be displayed with the checkbox.
    /// * `data` - User `Data` to be associated with the checkbox.
    /// * `flags` - [Flags][flags] modifying the behavior of the checkbox.
    /// * `indexes` - The optional index of the item in the `CheckboxTree`.
    ///
    /// `Returns` `-1` on error, `0` on success
    ///
    /// [flags]: crate::constants::checkboxtree
    /// [example]: #example
    ///
    pub fn add_item(&self, text: &str, data: D, flags: i32,
                    indexes: Option<&[i32]>) -> i32 {
        let mut i = 0;
        let mut c_array: Vec<i32>;
        if let Some(indexes) = indexes {
            c_array = Vec::with_capacity(indexes.len() + 1);
            while i < indexes.len() {
                c_array.push(indexes[i]);
                i += 1;
            }
            c_array.push(constants::ARG_LAST);
        } else {
            c_array = Vec::with_capacity(2);
            c_array.push(constants::ARG_APPEND);
            c_array.push(constants::ARG_LAST);
        }

        let c_str = CString::new(text).unwrap();
        unsafe {
            newtCheckboxTreeAddArray(self.co(), c_str.as_ptr(),
                                     data.newt_to_ptr(), flags,
                                     c_array.as_ptr() as *mut i32)
        }
    }

    ///
    /// Get the currently selected `CheckboxTree` item.
    ///
    /// `Returns` the user `Data` of the currently selected item.
    ///
    pub fn get_current(&self) -> Option<D> {
        let c_data = unsafe { newtCheckboxTreeGetCurrent(self.co()) };
        if c_data.is_null() { return None; }
        Some(D::newt_from_ptr(c_data))
    }

    ///
    /// Set the currently selected `CheckboxTree` item.
    ///
    /// * `data` - Associated user `Data` of the item to be selected.
    ///
    pub fn set_current(&self, data: D) {
        unsafe {
            newtCheckboxTreeSetCurrent(
                self.co(),
                data.newt_to_ptr() as *mut c_void
            );
        }
    }

    ///
    /// Get the currently selected checkbox items.
    ///
    /// `Returns` an array of the associated user `Data` of the selected
    /// checkboxes.
    ///
    pub fn get_selection(&self) -> Box<[D]> {
        let mut numitems: i32 = 0;
        let ptr = unsafe {
            newtCheckboxTreeGetSelection(self.co(), &mut numitems)
        };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    ///
    /// Select multiple checkbox items by their current checkbox value.
    ///
    /// The checkbox value is one of the characters set in the `sequence`
    /// parameter when the `CheckboxTree` is created with
    /// [`CheckboxTree::new()`]. It is possible to manually set a
    /// checkbox item to an out of sequence value with the function
    /// [`CheckboxTree::set_entry_value()`].
    ///
    /// * `seqval` - The `sequence` value by which to select checkbox items.
    ///
    /// `Returns` an array of the associated user `Data` of the selected
    /// checkboxes.
    ///
    pub fn get_multi_selection(&self, seqval: char) -> Box<[D]> {
        let mut numitems: i32 = 0;
        let ptr = unsafe {
            newtCheckboxTreeGetMultiSelection(
                self.co(),
                &mut numitems,
                seqval as c_char
            )
        };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    ///
    /// Find an item by its associated user `Data`.
    ///
    /// * `data` - The user `Data` associated with the item.
    ///
    /// `Returns` an array of integers representing the item's location
    /// index in the tree.
    ///
    pub fn find_item(&self, data: D) -> Box<[i32]> {
        let mut vec: Vec<i32> = Vec::new();
        unsafe {
            let rv = newtCheckboxTreeFindItem(
                self.co(),
                data.newt_to_ptr() as *mut c_void
            );

            let mut p = rv;
            let mut value: i32 = *p as i32;
            while value != constants::ARG_LAST {
                vec.push(value);
                p = p.offset(1);
                value = *p as i32;
            }
            libc::free(rv as *mut libc::c_void);
        }
        vec.into_boxed_slice()
    }

    ///
    /// Set an item's display text in the `CheckboxTree` by associated
    /// user `Data`.
    ///
    /// * `data` - The user `Data` associated with the checkbox item.
    /// * `text` - The new display text of the item.
    ///
    pub fn set_entry(&self, data: D, text: &str) {
        let c_str = CString::new(text).unwrap();
        unsafe {
            newtCheckboxTreeSetEntry(
                self.co(),
                data.newt_to_ptr(),
                c_str.as_ptr()
            );
        }
    }

    ///
    /// Get the current checkbox value of an item by associated user `Data`.
    ///
    /// The checkbox value may be one of the characters set in the `sequence`
    /// parameter when the `CheckboxTree` is created with
    /// [`CheckboxTree::new()`] or either [`CHECKBOXTREE_COLLPASED`][C] or
    /// [`CHECKBOXTREE_EXPANDED`][E] for tree nodes. It is possible to
    /// manually set a checkbox item to an out of sequence value with the
    /// function [`CheckboxTree::set_entry_value()`].
    ///
    /// * `data` - The user `Data` associated with the checkbox.
    ///
    /// `Returns` the current sequence value of the checkbox.
    ///
    /// [C]: crate::constants::checkboxtree::CHECKBOXTREE_COLLAPSED
    /// [E]: crate::constants::checkboxtree::CHECKBOXTREE_EXPANDED
    ///
    pub fn get_entry_value(&self, data: D) -> char {
        unsafe {
            newtCheckboxTreeGetEntryValue(
                self.co(),
                data.newt_to_ptr()
            ) as u8 as char
        }
    }

    ///
    /// Set the current checkbox value of an item by associated user `Data`.
    ///
    /// The checkbox value is generally a `char` defined by the `sequence`
    /// parameter when the `CheckboxTree` is created with
    /// [`CheckboxTree::new()`] but does not need to be.
    ///
    /// * `data` - The user `Data` associated with the checkbox.
    /// * `value` - The value to set the checkbox item to.
    ///
    pub fn set_entry_value(&self, data: D, value: char) {
        unsafe {
            newtCheckboxTreeSetEntryValue(
                self.co(),
                data.newt_to_ptr(),
                value as c_char
            );
        }
    }
}
