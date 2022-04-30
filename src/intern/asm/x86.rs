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
use std::ffi::{CStr,CString};
use std::mem::size_of;
use std::os::raw::{c_char,c_void};

use newt_sys::{newtComponent,newtGrid,newtGridElement,newtWinEntry};

use crate::Component;
use crate::constants::NEWT_GRID_EMPTY;
use crate::intern::funcs::*;
use crate::windows::WinEntry;

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

    let len = components.len();
    let types_ptr = types.as_ptr();
    let values_ptr = values.as_ptr();

   unsafe {
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
            inlateout("eax") NEWT_GRID_EMPTY => grid
       }
   }
   (grid, children)
}

pub fn button_bar_new(buttons: &[&str], buf: *mut newtComponent) -> newtGrid {
    let mut grid: newtGrid;

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);
    let buttons_ptr = button_ptrs.as_ptr();
    let len = buttons.len();

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
             in("ecx") len,
             in("edi") buf,
             out("eax") grid
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

    unsafe {
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
             inlateout("eax") list_item_ptr => rv
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

    let title = CString::new(title).unwrap();
    let title_ptr = title.as_ptr();

    let text  = CString::new(text).unwrap();
    let text_ptr = text.as_ptr();

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);

    let buttons_ptr = button_ptrs.as_ptr();
    let len = button_ptrs.len();

    let entries_buf: *mut newtWinEntry;
    let mut entries_text: Vec<CString> = Vec::new();

    let values_buf: *mut *mut c_char;
    let mut values_text: Vec<CString> = Vec::new();

    unsafe {
        let size = size_of::<newtWinEntry>() * (entries.len() + 1);
        entries_buf = libc::malloc(size) as *mut newtWinEntry;
        libc::memset(entries_buf as *mut c_void, 0, size);

        let size = size_of::<*mut c_char>() * (entries.len());
        values_buf = libc::malloc(size) as *mut *mut c_char;
        libc::memset(values_buf as *mut c_void, 0, size);

        for (cnt, entry) in entries.iter().enumerate() {
            let entry_buf = entries_buf.add(cnt);
            let value_buf = values_buf.add(cnt);
            let text = CString::new(entry.text.as_str()).unwrap();
            let value = CString::new(entry.value.as_str()).unwrap();
            *value_buf = value.as_ptr() as *mut c_char;

            (*entry_buf).text = text.as_ptr() as *mut c_char;
            (*entry_buf).value = value_buf;
            (*entry_buf).flags = entry.flags;
            entries_text.push(text);
            values_text.push(value);
        }

        let args: Vec<*const c_void> = vec![
            &entries_buf as *const _ as *const c_void,
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

             mov    esi, ebx
             mov    ecx, 7

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
             inlateout("eax") buttons_ptr => rv
        }

        for (cnt, entry) in entries.iter_mut().enumerate() {
            let buf = entries_buf.add(cnt);
            let value = CStr::from_ptr(*(*buf).value).to_str().unwrap();
            entry.value = String::from(value);
            libc::free(*(*buf).value as *mut c_void);
        }

        libc::free(entries_buf as *mut c_void);
        libc::free(values_buf as *mut c_void);
    }
    rv
}
