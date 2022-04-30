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

use libc::{c_char,c_void};
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

    unsafe {
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
             cmp    rcx, -1
             je     6f
             cmp    rcx, -2
             je     7f
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
             call   {func}

             shl    r12, 3
             add    rsp, r12
             jmp    8f

             6:
             mov    r8, rax
             jmp    4b

             7:
             mov    rdx, rax
             jmp    5b

             8:",

             func = in(reg) func,

             in("rdi") types_ptr,
             in("rsi") values_ptr,
             in("rcx") len,

             inlateout("rax") NEWT_GRID_EMPTY as usize => grid,

             out("rdx") _, out("r8") _, out("r9") _, out("r12") _,
             clobber_abi("sysv64")
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
            "std
             sub    rsp, 8
             xor    rax, rax
             push   rax
             mov    r12, 2

             lea    rdi, [rdi+rcx*8-8]
             lea    rsi, [rsi+rcx*8-8]

             sub    rcx, 3
             jz     3f
             cmp    rcx, -1
             je     6f
             cmp    rcx, -2
             je     7f
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
             call newtButtonBar

             shl    r12, 3
             add    rsp, r12
             jmp    8f

             6:
             xor    r8, r8
             jmp    4b

             7:
             xor    rdx, rdx
             jmp    5b

             8:",

             in("rdi") buf,
             in("rsi") buttons_ptr,
             in("rcx") buttons_len,
             out("rax") grid,

             out("rdx") _, out("r8") _, out("r9") _, out("r12") _,
             clobber_abi("sysv64")
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

             mov    rax, {list_item}
             push   rax
             mov    rax, {items_ptr}
             push   rax

             mov    rsi, {text_ptr}
             mov    rcx, {flex_down:r}

             call newtWinMenu

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

             out("rax") rv, out("r12") _,
             clobber_abi("sysv64")
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
    let text  = CString::new(text).unwrap();

    let buttons = str_slice_to_cstring_vec(buttons);
    let button_ptrs = cstring_vec_to_ptrs(&buttons);

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

             mov    rax, {entries_buf}
             push   rax

             mov    rsi, {text_ptr}
             mov    rcx, {flex_down:r}

             call newtWinEntries

             add    r12, 1
             shl    r12, 3
             add    rsp, r12",

             text_ptr = in(reg) text_ptr,
             flex_down = in(reg) flex_down,
             entries_buf = in(reg) entries_buf,

             in("rdi") title_ptr,
             in("rsi") buttons_ptr,
             in("rcx") buttons_len,
             in("rdx") suggested_width,
             in("r8") flex_up,
             in("r9") data_width,

             out("rax") rv, out("r12") _,
             clobber_abi("sysv64")
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
