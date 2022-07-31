//
// Copyright (C) 2022 Robert Gill <rtgill82@gmail.com>
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

use std::arch::asm;
use std::ffi::CString;
use std::os::raw::c_void;

use newt_sys::*;

use crate::Component;
use crate::constants::NEWT_GRID_EMPTY;
use crate::asm::windows::{WinEntry,WinEntryBuf};
use crate::asm::funcs::*;

pub fn grid_new<'t, 'a>(components: &'t [&'a dyn Component],
                        func: *const c_void)
  -> (newtGrid, Vec<&'a dyn Component>)
{
    let mut grid: newtGrid;
    let mut children = Vec::new();

    let mut types: Vec<newtGridElement> = Vec::new();
    let mut values: Vec<newtComponent> = Vec::new();
    for component in components.iter() {
        types.push(component.grid_element_type());
        values.push(component.co());
        children.push(*component);
    }

    unsafe {
        let types_ptr = types.as_ptr();
        let values_ptr = values.as_ptr();
        let len = components.len();

        asm! {
            "push   {{r0}}
             mov    r2, r0
             mov    r1, #2

             mov    r3, #4
             mov    r4, #-4
             mla    r0, r9, r3, r4
             add    r7, r7, r0
             add    r8, r8, r0

             subs   r9, r9, #2
             beq    3f
             cmp    r9, #-1
             beq    4f
             add    r1, r1, r9, LSL #1

             2:
             ldr    r0, [r8]
             push   {{r0}}
             sub    r8, r8, #4
             ldr    r0, [r7]
             push   {{r0}}
             sub    r7, r7, #4
             sub    r9, r9, #1
             bne    2b

             3:
             ldr    r3, [r8]
             sub    r8, r8, #4
             ldr    r2, [r7]
             sub    r7, r7, #4

             4:
             ldr    r1, [r8]
             ldr    r0, [r7]

             blx    r10

             mov    r1, r1, LSL #2
             add    r13, r1",

             in("r9") len,
             in("r10") func,

             inlateout("r0") NEWT_GRID_EMPTY as usize => grid,
             inlateout("r7") types_ptr => _,
             inlateout("r8") values_ptr => _,

             out("r1") _, out("r2") _, out("r3") _, out("r4") _,
             clobber_abi("C")
        }
    }
    (grid, children)
}

pub fn button_bar_new(buttons: &[&str], buf: *mut newtComponent) -> newtGrid {
    let mut grid: newtGrid;

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);
    let buttons_len = buttons.len();
    let buttons_ptr = button_ptrs.as_ptr();

    unsafe {
        asm! {
            "sub    r13, r13, #4
             mov    r0, #0
             push   {{r0}}
             mov    r10, #2

             mov    r8, #4
             mov    r9, #-4
             mla    r8, r7, r8, r9
             add    r4, r4, r8
             add    r5, r5, r8

             subs   r7, r7, #2
             beq    3f
             cmp    r7, #-1
             beq    5f
             mov    r0, r7, LSL #1
             add    r10, r0

             2:
             push   {{r4}}
             sub    r4, r4, #4
             ldr    r0, [r5]
             push   {{r0}}
             sub    r5, r5, #4
             subs   r7, r7, #1
             bne    2b

             3:
             mov    r3, r4
             sub    r4, r4, #4
             ldr    r2, [r5]
             sub    r5, r5, #4

             4:
             mov    r1, r4
             ldr    r0, [r5]

             bl     newtButtonBar
             b      6f

             5:
             mov    r2, #0
             b      4b

             6:
             mov    r10, r10, LSL #2
             add    r13, r10",

             inlateout("r7") buttons_len => _,
             inlateout("r4") buf => _,
             inlateout("r5") buttons_ptr => _,
             out("r0") grid,

             out("r1") _, out("r2") _, out("r3") _,
             out("r8") _, out("r9") _, out("r10") _,
             clobber_abi("C")
        }
    }
    grid
}

///
/// Open a window containing a [`Listbox`][listbox] menu.
///
/// _Requires that the `asm` feature be enabled._
///
/// * `title` - The window title.
/// * `text` - The message to display in the window.
/// * `suggested_width` - The preferred width for the window.
/// * `flex_down` - The minimum allowed difference between `suggested_width`
///                 and actual width.
/// * `flex_up` - The maximum allowed difference between `suggested_width`
///               and actual width.
/// * `max_list_height` - The maximum height to display the list of items.
/// * `items` - A slice containing the text for each item in the list.
/// * `buttons` - A slice containing the text for a number of buttons to display
///               in the window.
///
/// Returns a tuple pair as `(button, item)` where `button` is the button
/// number pressed to close the window and `item` is the item number in the
/// list that was selected.
///
/// [listbox]: ../widgets/struct.Listbox.html
///
#[allow(clippy::too_many_arguments)]
pub fn win_menu(title: &str, text: &str, suggested_width: i32, flex_down: i32,
                flex_up: i32, max_list_height: i32, items: &[&str],
                buttons: &[&str]) -> (i32, i32)
{
    let mut rv: i32;
    let mut list_item: i32 = 0;
    let list_item_ptr: *mut i32 = &mut list_item;

    let title = CString::new(title).unwrap();
    let text  = CString::new(text).unwrap();

    let items = str_slice_to_cstring_vec(items);
    let item_ptrs = cstring_vec_to_ptrs(&items);

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);

    let text_ptr = text.as_ptr();
    let title_ptr = title.as_ptr();
    let buttons_ptr = button_ptrs.as_ptr();
    let buttons_len = button_ptrs.len();
    let items_ptr = item_ptrs.as_ptr();

    let args: Vec<*const c_void> = vec![
        items_ptr as *const _ as *const c_void,
        max_list_height as *const c_void,
        flex_up as *const c_void
    ];
    let args_ptr = args.as_ptr();

    unsafe {
        asm! {
            "mov    r10, #4
             mov    r12, #-4
             mla    r10, r5, r10, r12
             add    r4, r4, r10
             mov    r10, r5
             tst    r5, #1
             bne    2f

             sub    r13, r13, #4
             add    r10, r10, #1

             2:
             ldr    r12, [r4]
             push   {{r12}}
             sub    r4, r4, #4
             subs   r5, #1
             bne    2b

             push   {{r7}}
             mov    r5, #3

             3:
             ldr    r12, [r8]
             push   {{r12}}
             add    r8, r8, #4
             subs   r5, #1
             bne    3b

             bl     newtWinMenu

             add    r10, r10, #4
             mov    r10, r10, LSL #2
             add    r13, r10",

             in("r1") text_ptr,
             in("r2") suggested_width,
             in("r3") flex_down,
             in("r5") buttons_len,
             in("r7") list_item_ptr,

             inlateout("r0") title_ptr => rv,
             inlateout("r4") buttons_ptr => _,
             inlateout("r8") args_ptr  => _,

             out("r10") _, out("r12") _,
             clobber_abi("C")
        }
    }
    (rv, list_item)
}

///
/// Open a window containing a number of text [`Entry`s][entry].
///
/// _Requires that the `asm` feature be enabled._
///
/// * `title` - The window title.
/// * `text` - The message to display in the window.
/// * `suggested_width` - The preferred width for the window.
/// * `flex_down` - The minimum allowed difference between `suggested_width`
///                 and actual width.
/// * `flex_up` - The maximum allowed difference between `suggested_width`
///               and actual width.
/// * `data_width` - The field width for all `Entry`s.
/// * `entries` - A slice containing a list of [`WinEntry`s][win_entry]
///               providing initial settings for each `Entry` field.
/// * `buttons` - A slice containing the text for a number of buttons to
///               display in the window.
///
/// Returns the number of the button pressed to close the window.
///
/// Each `WinEntry` in the `entries` array will be modified to contain the
/// data entered by the user. This can be accessed via the
/// [`WinEntry.value()`][win_entry_value] function.
///
/// [entry]: ../widgets/struct.Entry.html
/// [win_entry]: ../windows/struct.WinEntry.html
/// [win_entry_value]: ../windows/struct.WinEntry.html#method.value
///
#[allow(clippy::too_many_arguments)]
pub fn win_entries(title: &str, text: &str, suggested_width: i32,
                   flex_down: i32, flex_up: i32, data_width: i32,
                   entries: &mut [WinEntry], buttons: &[&str]) -> i32 {
    let mut rv: i32;
    let mut entries_buf = WinEntryBuf::new(entries);

    let title = CString::new(title).unwrap();
    let text  = CString::new(text).unwrap();

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);

    unsafe {
        let entries_ptr = entries_buf.as_mut_ptr();
        let title_ptr = title.as_ptr();
        let text_ptr = text.as_ptr();
        let buttons_ptr = button_ptrs.as_ptr();
        let buttons_len = button_ptrs.len();

        asm! {
            "mov    r7, #4
             mov    r8, #-4
             mla    r7, r5, r7, r8
             add    r4, r4, r7
             mov    r8, r5
             tst    r5, #1
             bne    2f

             sub    r13, r13, #4
             add    r8, r8, #1

             2:
             ldr    r7, [r4]
             push   {{r7}}
             sub    r4, r4, #4
             subs   r5, r5, #1
             bne    2b

             push   {{{entries_ptr}}}
             push   {{{data_width}}}
             push   {{{flex_up}}}

             bl     newtWinEntries

             add    r8, r8, #3
             mov    r8, r8, LSL #2
             add    r13, r8",

             entries_ptr = in(reg) entries_ptr,
             data_width = in(reg) data_width,
             flex_up = in(reg) flex_up,

             in("r1") text_ptr,
             in("r2") suggested_width,
             in("r3") flex_down,
             in("r5") buttons_len,

             inlateout("r0") title_ptr => rv,
             inlateout("r4") buttons_ptr => _,

             out("r7") _, out("r8") _,
             clobber_abi("C")
        }
    }
    rv
}
