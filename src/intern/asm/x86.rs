//
// Copyright (C) 2019 Robert Gill <locke@sdf.org>
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

use std::ffi::{CStr,CString};
use std::mem::size_of;
use libc::{c_char,c_void};

use newt_sys::{newtComponent,newtGrid,newtGridElement,newtWinEntry};
use crate::constants::NEWT_GRID_EMPTY;
use crate::intern::funcs::*;
use crate::windows::WinEntry;

pub fn grid_new(func: *const c_void, types: Vec<newtGridElement>,
                values: Vec<newtComponent>, len: usize) -> newtGrid {
   unsafe {
       let grid: newtGrid;
       llvm_asm! {
           "mov $3,      %ecx
            mov $2,      %esi
            mov $1,      %edi

            sub $$4,     %esp
            mov $4,      %eax
            push         %eax
            mov %ecx,    %ebx
            add $$1,     %ebx

            loop${:uid}:
            mov  (%esi), %eax
            push         %eax
            add $$4,     %esi
            mov  (%edi), %eax
            push         %eax
            add $$4,     %edi
            loop         loop${:uid}

            call         *$5
            mov %eax,    $0

            shl $$3,     %ebx
            add %ebx,    %esp"

           : "=r"(grid)
           : "m"(types.as_ptr()), "m"(values.as_ptr()),
             "m"(len), "i"(NEWT_GRID_EMPTY), "r"(func)
           : "esp", "eax", "ebx", "ecx", "edi", "esi"
       }
       grid
   }
}

pub fn button_bar_new(buttons: &[&str], buf: *mut newtComponent) -> newtGrid {
    let buttons = str_slice_to_cstring_vec(buttons);
    let mut button_ptrs = cstring_vec_to_ptrs(&buttons);
    button_ptrs.reverse();

    unsafe {
        let grid: newtGrid;
        llvm_asm!
        {
            "mov $3,      %ecx
             mov $2,      %esi
             mov $1,      %edi
             add $$4,     %edi

             sub $$4,     %esp
             xor %eax,    %eax
             push         %eax
             mov %ecx,    %ebx
             add $$1,     %ebx

             loop${:uid}:
             mov %esi,    %eax
             push         %eax
             add $$4,     %esi
             mov (%edi),  %eax
             push         %eax
             add $$4,     %edi
             loop         loop${:uid}

             call newtButtonBar
             mov %eax,    $0

             shl $$3,     %ebx
             add %ebx,    %esp"

            : "=r"(grid)
            : "m"(button_ptrs.as_ptr()), "m"(buf), "m"(buttons.len())
            : "esp", "eax", "ebx", "ecx", "edi", "esi"

        }
        grid
    }
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
                buttons: &[&str]) -> (i32, i32) {
    let mut rv: i32;
    let list_item: i32 = 0;

    let title = CString::new(title).unwrap();
    let text  = CString::new(text).unwrap();

    let items = str_slice_to_cstring_vec(items);
    let item_ptrs = cstring_vec_to_ptrs(&items);

    let buttons = str_slice_to_cstring_vec(buttons);
    let mut button_ptrs = cstring_vec_to_ptrs(&buttons);
    button_ptrs.reverse();

    unsafe {
        llvm_asm! {
            "mov $10,    %ecx
             mov $9,     %esi
             mov %ecx,   %ebx

             test $$1,   %ecx
             jz          loop${:uid}

             sub $$4,    %esp
             add $$1,    %ebx

             loop${:uid}:
             mov (%esi), %eax
             push        %eax
             add $$4,    %esi
             loop        loop${:uid}

             mov $8,     %eax
             push        %eax
             mov $7,     %eax
             push        %eax
             mov $6,     %eax
             push        %eax
             mov $5,     %eax
             push        %eax
             mov $4,     %eax
             push        %eax
             mov $3,     %eax
             push        %eax
             mov $2,     %eax
             push        %eax
             mov $1,     %eax
             push        %eax

             call newtWinMenu
             mov %eax,   $0

             add $$8,    %ebx
             shl $$2,    %ebx
             add %ebx,   %esp"

            : "=r"(rv)
            : "m"(title.as_ptr()), "m"(text.as_ptr()), "m"(suggested_width),
              "m"(flex_down), "m"(flex_up), "m"(max_list_height),
              "m"(item_ptrs.as_ptr()), "m"(&list_item),
              "m"(button_ptrs.as_ptr()), "m"(button_ptrs.len())
            : "esp", "eax", "ebx", "ecx", "esi"
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
    let mut button_ptrs = cstring_vec_to_ptrs(&buttons);
    button_ptrs.reverse();

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

        llvm_asm! {
            "mov $9,     %ecx
             mov $8,     %esi
             mov %ecx,   %ebx

             test $$1,   %ecx
             jnz         loop${:uid}

             sub $$4,    %esp
             add $$1,    %ebx

             loop${:uid}:
             mov (%esi), %eax
             push        %eax
             add  $$4,   %esi
             loop        loop${:uid}

             mov $7,     %eax
             push        %eax
             mov $6,     %eax
             push        %eax
             mov $5,     %eax
             push        %eax
             mov $4,     %eax
             push        %eax
             mov $3,     %eax
             push        %eax
             mov $2,     %eax
             push        %eax
             mov $1,     %eax
             push        %eax

             call newtWinEntries
             mov  %eax,   $0

             add $$7,    %ebx
             shl $$2,    %ebx
             add %ebx,   %esp"

            : "=r"(rv)
            : "m"(title.as_ptr()), "m"(text.as_ptr()), "m"(suggested_width),
              "m"(flex_down), "m"(flex_up), "m"(data_width), "m"(entries_buf),
              "m"(button_ptrs.as_ptr()),  "m"(button_ptrs.len())
            : "esp", "eax", "ebx", "ecx", "esi"
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
