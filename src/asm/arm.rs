//
// Copyright (C) 2022,2025 Robert Gill <rtgill82@gmail.com>
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
use crate::constants::GRID_EMPTY;
use crate::asm::windows::{WinEntry,WinEntryBuf};
use crate::asm::funcs::*;

#[inline]
pub unsafe fn
grid_new<'t, 'a>(components: &'t [&'a dyn Component],
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

    let types_ptr = types.as_ptr();
    let values_ptr = values.as_ptr();
    let len = components.len();

    asm! {
        "push   {{r0}}
         mov    r2, r0
         mov    r5, #1

         mov    r3, #4
         mov    r4, #-4
         mla    r0, r9, r3, r4
         add    r7, r7, r0
         add    r8, r8, r0

         subs   r9, r9, #2
         beq    3f
         cmp    r9, #-1
         beq    4f
         add    r5, r5, r9, LSL #1

         2:
         ldr    r0, [r8], #-4
         push   {{r0}}
         ldr    r0, [r7], #-4
         push   {{r0}}
         subs   r9, r9, #1
         bne    2b

         3:
         ldr    r3, [r8], #-4
         ldr    r2, [r7], #-4

         4:
         ldr    r1, [r8]
         ldr    r0, [r7]

         blx    r10

         mov    r5, r5, LSL #2
         add    sp, r5",

         inlateout("r0") GRID_EMPTY as *const c_void => grid,
         inlateout("r7") types_ptr => _,
         inlateout("r8") values_ptr => _,
         inlateout("r9") len => _,
         inlateout("r10") func => _,

         out("r1") _, out("r2") _, out("r3") _, out("r4") _, out("r5") _,
         out("ip") _, out("lr") _
    }
    (grid, children)
}

#[inline]
pub unsafe fn
button_bar_new(buttons: &[&str], buf: *mut newtComponent) -> newtGrid
{
    let mut grid: newtGrid;

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);
    let buttons_len = buttons.len();
    let buttons_ptr = button_ptrs.as_ptr();

    asm! {
        "sub    sp, sp, #4
         mov    r0, #0
         push   {{r0}}

         mov    r8, #4
         mov    r9, #-4
         mla    r8, r7, r8, r9
         add    r4, r4, r8
         add    r5, r5, r8
         mov    r9, #2

         subs   r7, r7, #2
         beq    3f

         mov    r2, #0
         cmp    r7, #-1
         beq    4f

         mov    r0, r7, LSL #1
         add    r9, r0

         2:
         push   {{r5}}
         sub    r5, r5, #4
         ldr    r0, [r4], #-4
         push   {{r0}}
         subs   r7, r7, #1
         bne    2b

         3:
         mov    r3, r5
         sub    r5, r5, #4
         ldr    r2, [r4], #-4

         4:
         mov    r1, r5
         ldr    r0, [r4]

         bl     newtButtonBar

         mov    r9, r9, LSL #2
         add    sp, r9",

         inlateout("r4") buttons_ptr => _,
         inlateout("r5") buf => _,
         inlateout("r7") buttons_len => _,

         out("r0") grid,

         out("r1") _, out("r2") _, out("r3") _, out("r8") _, out("r9") _,
         out("ip") _, out("lr") _
    }
    grid
}

#[inline]
#[allow(clippy::too_many_arguments)]
pub unsafe fn
win_menu_new(title: &str, text: &str, suggested_width: i32, flex_down: i32,
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

    asm! {
        "mov    r9, #4
         mov    r10, #-4
         mla    r9, r5, r9, r10
         add    r4, r4, r9
         mov    r9, r5
         tst    r5, #1
         bne    2f

         sub    sp, sp, #4
         add    r9, r9, #1

         2:
         ldr    r10, [r4], #-4
         push   {{r10}}
         subs   r5, #1
         bne    2b

         push   {{r7}}
         mov    r5, #3

         3:
         ldr    r10, [r8], #-4
         push   {{r10}}
         subs   r5, #1
         bne    3b

         bl     newtWinMenu

         add    r9, r9, #4
         mov    r9, r9, LSL #2
         add    sp, r9",

         inlateout("r0") title_ptr => rv,
         inlateout("r1") text_ptr => _,
         inlateout("r2") suggested_width => _,
         inlateout("r3") flex_down => _,

         inlateout("r4") buttons_ptr => _,
         inlateout("r5") buttons_len => _,
         inlateout("r7") list_item_ptr => _,
         inlateout("r8") args_ptr  => _,

         out("r9") _, out("r10") _,
         out("ip") _, out("lr") _
    }
    (rv, list_item)
}

#[inline]
#[allow(clippy::too_many_arguments)]
pub unsafe fn
win_entries_new(title: &str, text: &str, suggested_width: i32,
                flex_down: i32, flex_up: i32, data_width: i32,
                entries: &mut [WinEntry], buttons: &[&str]) -> i32
{
    let mut rv: i32;
    let mut entries_buf = WinEntryBuf::new(entries);

    let title = CString::new(title).unwrap();
    let text  = CString::new(text).unwrap();

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);

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

         sub    sp, sp, #4
         add    r8, r8, #1

         2:
         ldr    r7, [r4], #-4
         push   {{r7}}
         subs   r5, r5, #1
         bne    2b

         push   {{{entries_ptr}}}
         push   {{{data_width}}}
         push   {{{flex_up}}}

         bl     newtWinEntries

         add    r8, r8, #3
         mov    r8, r8, LSL #2
         add    sp, r8",

         entries_ptr = in(reg) entries_ptr,
         data_width = in(reg) data_width,
         flex_up = in(reg) flex_up,

         inlateout("r0") title_ptr => rv,
         inlateout("r1") text_ptr => _,
         inlateout("r2") suggested_width => _,
         inlateout("r3") flex_down => _,
         inlateout("r4") buttons_ptr => _,
         inlateout("r5") buttons_len => _,

         out("r7") _, out("r8") _,
         out("ip") _, out("lr") _
    }
    rv
}
