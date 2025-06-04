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
        "addi   sp, sp, -8
         sd     a0, (sp)
         li     s2, 1

         li     t5, 4
         mul    t4, t2, t5
         addi   t4, t4, -4
         add    t0, t0, t4

         li     t5, 8
         mul    t4, t2, t5
         addi   t4, t4, -8
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
         ld     a0, (t1)
         addi   t1, t1, -8
         addi   sp, sp, -8
         sd     a0, (sp)
         lw     a0, (t0)
         addi   t0, t0, -4
         addi   sp, sp, -8
         sd     a0, (sp)
         addi   t2, t2, -1
         bne    t2, zero, 2b

         3:
         ld     a7, (t1)
         addi   t1, t1, -8
         lw     a6, (t0)
         addi   t0, t0, -4

         4:
         ld     a5, (t1)
         addi   t1, t1, -8
         lw     a4, (t0)
         addi   t0, t0, -4

         5:
         ld     a3, (t1)
         addi   t1, t1, -8
         lw     a2, (t0)
         addi   t0, t0, -4

         6:
         ld     a1, (t1)
         lw     a0, (t0)

         jalr   ra, t3

         slli   s2, s2, 3
         add    sp, sp, s2",

         inlateout("a0") GRID_EMPTY as *const c_void => grid,

         inlateout("t0") types_ptr => _,
         inlateout("t1") values_ptr => _,
         inlateout("t2") len => _,
         inlateout("t3") func => _,

         out("s2") _, clobber_abi("C")
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
        "addi   sp, sp, -8
         sd     zero, (sp)
         li     s2, 1

         li     t3, 8
         mul    t4, t2, t3
         addi   t4, t4, -8
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
         addi   sp, sp, -8
         sd     t1, (sp)
         addi   t1, t1, -8
         ld     t3, (t0)
         addi   t0, t0, -8
         addi   sp, sp, -8
         sd     t3, (sp)
         addi   t2, t2, -1
         bne    t2, zero, 2b

         3:
         mv     a7, t1
         addi   t1, t1, -8
         ld     a6, (t0)
         addi   t0, t0, -8

         4:
         mv     a5, t1
         addi   t1, t1, -8
         ld     a4, (t0)
         addi   t0, t0, -8

         5:
         mv     a3, t1
         addi   t1, t1, -8
         ld     a2, (t0)
         addi   t0, t0, -8

         6:
         mv     a1, t1
         ld     a0, (t0)

         call   newtButtonBar

         slli   s2, s2, 3
         add    sp, sp, s2",

         inlateout("t0") buttons_ptr => _,
         inlateout("t1") buf => _,
         inlateout("t2") buttons_len => _,

         out("a0") grid,
         out("s2") _, clobber_abi("C")
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

    asm! {
        "li     t3, 1
         li     s2, 8
         mul    s2, s2, t2
         addi   s2, s2, -8
         add    t1, t1, s2

         mv     s2, t2
         andi   t4, t2, 1
         beq    t4, t3, 2f

         addi   sp, sp, -8
         addi   s2, s2, 1

         2:
         ld     a0, (t1)
         addi   t1, t1, -8
         addi   sp, sp, -8
         sd     a0, (sp)
         addi   t2, t2, -1
         bne    t2, zero, 2b

         mv     a0, t0
         call   newtWinMenu

         slli   s2, s2, 3
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
        "li     t3, 1
         beq    t2, t3, 3f

         li     s2, 8
         mul    s2, s2, t2
         addi   s2, s2, -8
         add    t1, t1, s2
         addi   t2, t2, -1

         mv     s2, t2
         andi   t4, t2, 1
         beq    t4, t3, 2f

         addi   sp, sp, -8
         addi   s2, s2, 1

         2:
         ld     a7, (t1)
         addi   t1, t1, -8
         addi   sp, sp, -8
         sd     a7, (sp)
         addi   t2, t2, -1
         bne    t2, zero, 2b

         3:
         ld     a7, (t1)
         mv     a0, t0
         call   newtWinEntries

         slli   s2, s2, 3
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
    rv
}
