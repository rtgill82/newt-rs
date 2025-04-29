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
            "addi   sp, sp, -4
             sw     a0, (sp)
             li     s2, 1

             li     t5, 4
             mul    t4, t2, t5
             addi   t4, t4, -4
             add    t0, t0, t4
             add    t1, t1, t4

             addi   t2, t2, -4
             beq    t2, zero, 3f

             mv     a6, a0
             li     t4, -1
             beq    t2, t4, 4f

             mv     a4, a0
             li     t4, -2
             beq    t2, t4, 5f

             mv     a2, a0
             li     t4, -3
             beq    t2, t4, 6f
             slli   t4, t2, 1
             add    s2, s2, t4

             2:
             lw     a0, (t1)
             addi   t1, t1, -4
             addi   sp, sp, -4
             sw     a0, (sp)
             lw     a0, (t0)
             addi   t0, t0, -4
             addi   sp, sp, -4
             sw     a0, (sp)
             addi   t2, t2, -1
             bne    t2, zero, 2b

             3:
             lw     a7, (t1)
             addi   t1, t1, -4
             lw     a6, (t0)
             addi   t0, t0, -4

             4:
             lw     a5, (t1)
             addi   t1, t1, -4
             lw     a4, (t0)
             addi   t0, t0, -4

             5:
             lw     a3, (t1)
             addi   t1, t1, -4
             lw     a2, (t0)
             addi   t0, t0, -4

             6:
             lw     a1, (t1)
             lw     a0, (t0)

             jalr   ra, t3

             slli   s2, s2, 2
             add    sp, sp, s2",

             inlateout("a0") NEWT_GRID_EMPTY as *const c_void => grid,

             inlateout("t0") types_ptr => _,
             inlateout("t1") values_ptr => _,
             inlateout("t2") len => _,
             inlateout("t3") func => _,

             out("s2") _, clobber_abi("C")
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
            "addi   sp, sp, -4
             sw     zero, (sp)
             li     s2, 1

             li     t3, 4
             mul    t4, t2, t3
             addi   t4, t4, -4
             add    t0, t0, t4
             add    t1, t1, t4

             addi   t2, t2, -4
             beq    t2, zero, 3f

             mv     a6, zero
             li     t3, -1
             beq    t2, t3, 4f

             mv     a4, zero
             li     t3, -2
             beq    t2, t3, 5f

             mv     a2, zero
             li     t3, -3
             beq    t2, t3, 6f
             slli   t3, t2, 1
             add    s2, s2, t3

             2:
             addi   sp, sp, -4
             sw     t1, (sp)
             addi   t1, t1, -4
             lw     t3, (t0)
             addi   t0, t0, -4
             addi   sp, sp, -4
             sw     t3, (sp)
             addi   t2, t2, -1
             bne    t2, zero, 2b

             3:
             mv     a7, t1
             addi   t1, t1, -4
             lw     a6, (t0)
             addi   t0, t0, -4

             4:
             mv     a5, t1
             addi   t1, t1, -4
             lw     a4, (t0)
             addi   t0, t0, -4

             5:
             mv     a3, t1
             addi   t1, t1, -4
             lw     a2, (t0)
             addi   t0, t0, -4

             6:
             mv     a1, t1
             lw     a0, (t0)

             call   newtButtonBar

             slli   s2, s2, 2
             add    sp, sp, s2",

             inlateout("t0") buttons_ptr => _,
             inlateout("t1") buf => _,
             inlateout("t2") buttons_len => _,

             out("a0") grid,
             out("s2") _, clobber_abi("C")
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
            "li     t3, 1
             li     s2, 4
             mul    s2, s2, t2
             addi   s2, s2, -4
             add    t1, t1, s2

             mv     s2, t2
             andi   t4, t2, 1
             beq    t4, t3, 2f

             addi   sp, sp, -4
             addi   s2, s2, 1

             2:
             lw     a0, (t1)
             addi   t1, t1, -4
             addi   sp, sp, -4
             sw     a0, (sp)
             addi   t2, t2, -1
             bne    t2, zero, 2b

             mv     a0, t0
             call   newtWinMenu

             slli   s2, s2, 2
             add    sp, sp, s2",

             in("t0")        title_ptr,
             inlateout("a1") text_ptr => _,
             inlateout("a2") suggested_width => _,
             inlateout("a3") flex_down => _,
             inlateout("a4") flex_up => _,
             inlateout("a5") max_list_height => _,
             inlateout("a6") items_ptr => _,
             inlateout("a7") list_item_ptr => _,

             inlateout("t1") buttons_ptr => _,
             inlateout("t2") buttons_len => _,

             out("a0") rv,
             out("s2") _, clobber_abi("C")
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
            "li     t3, 1
             beq    t2, t3, 3f

             li     s2, 4
             mul    s2, s2, t2
             addi   s2, s2, -4
             add    t1, t1, s2
             addi   t2, t2, -1

             mv     s2, t2
             andi   t4, t2, 1
             beq    t4, t3, 2f

             addi   sp, sp, -4
             addi   s2, s2, 1

             2:
             lw     a7, (t1)
             addi   t1, t1, -4
             addi   sp, sp, -4
             sw     a7, (sp)
             addi   t2, t2, -1
             bne    t2, zero, 2b

             3:
             lw     a7, (t1)
             mv     a0, t0
             call   newtWinEntries

             slli   s2, s2, 2
             add    sp, sp, s2",

             in("t0")        title_ptr,
             inlateout("a1") text_ptr => _,
             inlateout("a2") suggested_width => _,
             inlateout("a3") flex_down => _,
             inlateout("a4") flex_up => _,
             inlateout("a5") data_width => _,
             inlateout("a6") entries_ptr => _,

             inlateout("t1") buttons_ptr => _,
             inlateout("t2") buttons_len => _,

             out("a0") rv,
             out("s2") _, clobber_abi("C")
        }
    }

    rv
}
