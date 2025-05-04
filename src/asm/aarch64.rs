//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
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
            "stp    x0, x0, [sp, #-16]!
             mov    x20, #2

             mov    x2, #4
             mul    x1, x12, x2
             sub    x1, x1, #4
             add    x10, x10, x1

             mov    x2, #8
             mul    x1, x12, x2
             sub    x1, x1, #8
             add    x11, x11, x1

             subs   x12, x12, #4
             beq    3f

             mov    x6, x0
             cmp    x12, #-1
             beq    4f

             mov    x4, x0
             cmp    x12, #-2
             beq    5f

             mov    x2, x0
             cmp    x12, #-3
             beq    6f
             add    x20, x20, x12, LSL #1

             2:
             ldr    x0, [x11], #-8
             ldr    x1, [x10], #-4
             stp    x0, x1, [sp, #-16]!
             subs   x12, x12, #1
             bne    2b

             3:
             ldr    x7, [x11], #-8
             ldr    w6, [x10], #-4

             4:
             ldr    x5, [x11], #-8
             ldr    w4, [x10], #-4

             5:
             ldr    x3, [x11], #-8
             ldr    w2, [x10], #-4

             6:
             ldr    x1, [x11]
             ldr    w0, [x10]

             blr    x13

             lsl    x20, x20, #3
             add    sp, sp, x20",

             inlateout("x0") NEWT_GRID_EMPTY as *const c_void => grid,

             inlateout("x10") types_ptr => _,
             inlateout("x11") values_ptr => _,
             inlateout("x12") len => _,
             inlateout("x13") func => _,

             out("x20") _, clobber_abi("C")
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
            "stp    xzr, xzr, [sp, #-16]!
             mov    x20, #2

             mov    x2, #8
             mul    x1, x12, x2
             sub    x1, x1, #8
             add    x10, x10, x1
             add    x11, x11, x1

             subs   x12, x12, #4
             beq    3f

             mov    x6, xzr
             cmp    x12, #-1
             beq    4f

             mov    x4, xzr
             cmp    x12, #-2
             beq    5f

             mov    x2, xzr
             cmp    x12, #-3
             beq    6f
             add    x20, x20, x12, LSL #1

             2:
             ldr    x0, [x10], #-8
             stp    x0, x11, [sp, #-16]!
             sub    x11, x11, #8
             subs   x12, x12, #1
             bne    2b

             3:
             mov    x7, x11
             sub    x11, x11, #8
             ldr    x6, [x10], #-8

             4:
             mov    x5, x11
             sub    x11, x11, #8
             ldr    x4, [x10], #-8

             5:
             mov    x3, x11
             sub    x11, x11, #8
             ldr    x2, [x10], #-8

             6:
             mov    x1, x11
             ldr    x0, [x10]

             bl     newtButtonBar

             lsl    x20, x20, #3
             add    sp, sp, x20",

             inlateout("x10") buttons_ptr => _,
             inlateout("x11") buf => _,
             inlateout("x12") buttons_len => _,

             out("x0") grid,
             out("x20") _, clobber_abi("C")
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

    unsafe {
        asm! {
            "mov    x20, #8
             mul    x20, x20, x11
             sub    x20, x20, #8
             add    x10, x10, x20

             mov    x20, x11
             tst    x11, #1
             beq    2f

             ldr    x0, [x10], #-8
             stp    x0, xzr, [sp, #-16]!
             add    x20, x20, #1
             subs   x11, x11, #1
             beq    3f

             2:
             ldr    x12, [x10], #-8
             ldr    x13, [x10], #-8
             stp    x12, x13, [sp, #-16]!
             subs   x11, x11, #2
             bne    2b

             3:
             mov    x0, {title_ptr}
             bl     newtWinMenu

             lsl    x20, x20, #3
             add    sp, sp, x20",

             title_ptr = in(reg) title_ptr,

             inlateout("x1") text_ptr => _,
             inlateout("x2") suggested_width => _,
             inlateout("x3") flex_down => _,
             inlateout("x4") flex_up => _,
             inlateout("x5") max_list_height => _,
             inlateout("x6") items_ptr => _,
             inlateout("x7") list_item_ptr => _,

             inlateout("x10") buttons_ptr => _,
             inlateout("x11") buttons_len => _,

             out("x0") rv,
             out("x12") _, out("x13") _, out("x20") _,
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
            "cmp    x11, #1
             beq    3f

             mov    x20, #8
             mul    x20, x20, x11
             sub    x20, x20, #8
             add    x10, x10, x20
             sub    x11, x11, #1

             mov    x20, x11
             tst    x11, #1
             beq    2f

             ldr    x0, [x10], #-8
             stp    x0, xzr, [sp, #-16]!
             add    x20, x20, #1

             2:
             ldr    x12, [x10], #-8
             ldr    x13, [x10], #-8
             stp    x12, x13, [sp, #-16]!
             subs   x11, x11, #2
             bne    2b

             3:
             ldr    x7, [x10]
             mov    x0, {title_ptr}
             bl     newtWinEntries

             lsl    x20, x20, #3
             add    sp, sp, x20",

             title_ptr = in(reg) title_ptr,

             inlateout("x1") text_ptr => _,
             inlateout("x2") suggested_width => _,
             inlateout("x3") flex_down => _,
             inlateout("x4") flex_up => _,
             inlateout("x5") data_width => _,
             inlateout("x6") entries_ptr => _,

             inlateout("x10") buttons_ptr => _,
             inlateout("x11") buttons_len => _,

             out("x0") rv,
             out("x12") _, out("x13") _, out("x20") _,
             clobber_abi("C")
        }
    }

    rv
}
