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
        "std
         sub    rsp, 8
         push   rax
         mov    r12, 2

         lea    rdi, [rdi+rcx*4-4]
         lea    rsi, [rsi+rcx*8-8]

         sub    rcx, 3
         jz     3f

         mov    r8, rax
         cmp    rcx, -1
         je     4f

         mov    rdx, rax
         cmp    rcx, -2
         je     5f

         mov    rax, rcx
         shl    rax, 1
         add    r12, rax

         2:
         lodsq
         push   rax
         mov    eax, [rdi]
         push   rax
         sub    rdi, 4
         loop   2b

         3:
         mov    r8, [rsi]
         sub    rsi, 8
         mov    r9d, [rdi]
         sub    rdi, 4

         4:
         mov    rcx, [rsi]
         sub    rsi, 8
         mov    edx, [rdi]
         sub    rdi, 4

         5:
         mov    rsi, [rsi]
         mov    edi, [rdi]

         cld
         call   r10

         shl    r12, 3
         add    rsp, r12",

         in("rdi") types_ptr,
         in("rsi") values_ptr,
         in("rcx") len,
         in("r10") func,

         inlateout("rax") GRID_EMPTY as *const c_void => grid,
         out("r12") _, clobber_abi("C")
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
        "std
         sub    rsp, 8
         xor    rax, rax
         push   rax
         mov    r12, 2

         lea    rdi, [rdi+rcx*8-8]
         lea    rsi, [rsi+rcx*8-8]

         sub    rcx, 3
         jz     3f

         xor    r8, r8
         cmp    rcx, -1
         je     4f

         xor    rdx, rdx
         cmp    rcx, -2
         je     5f

         mov    rax, rcx
         shl    rax, 1
         add    r12, rax

         2:
         mov    rax, rdi
         push   rax
         sub    rdi, 8
         lodsq
         push   rax
         loop   2b

         3:
         mov    r9, rdi
         sub    rdi, 8
         mov    r8, [rsi]
         sub    rsi, 8

         4:
         mov    rcx, rdi
         sub    rdi, 8
         mov    rdx, [rsi]
         sub    rsi, 8

         5:
         xchg   rdi, rsi
         mov    rdi, [rdi]

         cld
         call   newtButtonBar

         shl    r12, 3
         add    rsp, r12",

         in("rdi") buf,
         in("rsi") buttons_ptr,
         in("rcx") buttons_len,
         out("rax") grid,

         out("r12") _, clobber_abi("C")
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
        "std
         mov    r12, rcx
         lea    rsi, [rsi+rcx*8-8]
         test   rcx, 1
         jz     2f

         sub    rsp, 8
         inc    r12

         2:
         lodsq
         push   rax
         loop   2b
         cld

         push   {list_item}
         push   {items_ptr}

         mov    rsi, {text_ptr}
         mov    rcx, {flex_down:r}

         call   newtWinMenu

         add    r12, 2
         shl    r12, 3
         add    rsp, r12",

         list_item = in(reg) list_item_ptr,
         items_ptr = in(reg) items_ptr,
         text_ptr  = in(reg) text_ptr,
         flex_down = in(reg) flex_down,

         in("rdi") title_ptr,
         in("rsi") buttons_ptr,
         in("rcx") buttons_len,
         in("rdx") suggested_width,
         in("r8") flex_up,
         in("r9") max_list_height,

         out("rax") rv,

         out("r12") _, clobber_abi("C")
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
        "std
         mov    r12, rcx
         lea    rsi, [rsi+rcx*8-8]
         test   rcx, 1
         jnz    2f

         sub    rsp, 8
         inc    r12

         2:
         lodsq
         push   rax
         loop   2b
         cld

         push   {entries_ptr}
         mov    rsi, {text_ptr}
         mov    rcx, {flex_down:r}

         call   newtWinEntries

         add    r12, 1
         shl    r12, 3
         add    rsp, r12",

         text_ptr = in(reg) text_ptr,
         flex_down = in(reg) flex_down,
         entries_ptr = in(reg) entries_ptr,

         in("rdi") title_ptr,
         in("rsi") buttons_ptr,
         in("rcx") buttons_len,
         in("rdx") suggested_width,
         in("r8") flex_up,
         in("r9") data_width,

         out("rax") rv,

         out("r12") _, clobber_abi("C")
    }
    rv
}
