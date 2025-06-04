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

    let len = components.len();
    let types_ptr = types.as_ptr();
    let values_ptr = values.as_ptr();

    asm! {
        "std
         push    esi
         push    ebp
         mov     ebp, esp

         push    eax
         mov     esi, ebx
         lea     edi, [edi+ecx*4-4]
         lea     esi, [esi+ecx*4-4]

         2:
         lodsd
         push    eax
         mov     eax, [edi]
         push    eax
         sub     edi, 4
         loop    2b
         cld

         call    edx

         mov     esp, ebp
         pop     ebp
         pop     esi",

         in("ebx") values_ptr,
         in("ecx") len,
         in("edx") func,
         in("edi") types_ptr,
         inlateout("eax") GRID_EMPTY as *const c_void => grid,

         clobber_abi("C")
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
    let buttons_ptr = button_ptrs.as_ptr();
    let buttons_len = buttons.len();

    unsafe {
        asm!
        {
            "std
             push   esi
             push   ebp
             mov    ebp, esp

             mov    esi, ebx
             sub    esp, 4
             xor    eax, eax
             push   eax

             lea    edi, [edi+ecx*4-4]
             lea    esi, [esi+ecx*4-4]

             2:
             mov    eax, edi
             push   eax
             sub    edi, 4
             lodsd
             push   eax
             loop   2b
             cld

             call   newtButtonBar

             mov    esp, ebp
             pop    ebp
             pop    esi",

             in("ebx") buttons_ptr,
             in("ecx") buttons_len,
             in("edi") buf,
             out("eax") grid,

             clobber_abi("C")
        }
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

    let title_ptr = title.as_ptr();
    let text_ptr = text.as_ptr();

    let items = str_slice_to_cstring_vec(items);
    let item_ptrs = cstring_vec_to_ptrs(&items);
    let items_ptr = item_ptrs.as_ptr();

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);

    let buttons_ptr = button_ptrs.as_ptr();
    let len = button_ptrs.len();

    let args: Vec<*const c_void> = vec![
        &items_ptr as *const _ as *const c_void,
        &max_list_height as *const _ as *const c_void,
        &flex_up as *const _ as *const c_void,
        &flex_down as *const _ as *const c_void,
        &suggested_width as *const _ as *const c_void,
        &text_ptr as *const _ as *const c_void,
        &title_ptr as *const _ as *const c_void
    ];
    let args_ptr = args.as_ptr();

    asm! {
        "std
         push   esi
         push   ebp
         mov    ebp, esp

         push   edx
         push   eax

         mov    esi, [ebp-4]
         lea    esi, [esi+ecx*4-4]
         test   ecx, 1
         jz     2f

         sub    esp, 4

         2:
         lodsd
         push   eax
         loop   2b
         cld

         mov    eax, [ebp-8]
         push   eax

         mov    esi, ebx
         mov    ecx, 7

         3:
         lodsd
         mov    eax, [eax]
         push   eax
         loop   3b

         call   newtWinMenu

         mov    esp, ebp
         pop    ebp
         pop    esi",

         in("ebx") args_ptr,
         in("ecx") len,
         in("edx") buttons_ptr,
         inlateout("eax") list_item_ptr => rv,

         clobber_abi("C")
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
    let title_ptr = title.as_ptr();

    let text  = CString::new(text).unwrap();
    let text_ptr = text.as_ptr();

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);

    let buttons_ptr = button_ptrs.as_ptr();
    let len = button_ptrs.len();

    let entries_ptr = entries_buf.as_mut_ptr();
    let args: Vec<*const c_void> = vec![
        &data_width as *const _ as *const c_void,
        &flex_up as *const _ as *const c_void,
        &flex_down as *const _ as *const c_void,
        &suggested_width as *const _ as *const c_void,
        &text_ptr as *const _ as *const c_void,
        &title_ptr as *const _ as *const c_void
    ];
    let args_ptr = args.as_ptr();

    asm! {
        "std
         push   esi
         push   ebp
         mov    ebp, esp

         mov    esi, eax
         lea    esi, [esi+ecx*4-4]
         test   ecx, 1
         jz     2f

         sub    esp, 4

         2:
         mov    eax, [esi]
         push   eax
         sub    esi, 4
         loop   2b
         cld

         push   edx
         mov    esi, ebx
         mov    ecx, 6

         4:
         lodsd
         mov    eax, [eax]
         push   eax
         loop   4b

         call   newtWinEntries

         mov    esp, ebp
         pop    ebp
         pop    esi",

         in("ebx") args_ptr,
         in("ecx") len,
         in("edx") entries_ptr,
         inlateout("eax") buttons_ptr => rv,

         clobber_abi("C")
    }
    rv
}
