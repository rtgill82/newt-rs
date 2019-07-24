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
        asm! {
            "movq $3,     %rcx
             movq $2,     %rsi
             movq $1,     %rdi

             subq $$8,    %rsp
             movq $4,     %rax
             pushq        %rax
             xorq %rbx,   %rbx
             addq $$2,    %rbx

             sub $$3,     %rcx
             jz           reg0${:uid}
             cmp $$-1,    %rcx
             je           null_r8${:uid}
             cmp $$-2,    %rcx
             je           null_rdx${:uid}
             movq %rcx,   %rax
             shl $$1,     %rax
             addq %rax,   %rbx

             loop${:uid}:
             movq (%rsi), %rax
             pushq        %rax
             addq $$8,    %rsi
             movl (%rdi), %eax
             pushq        %rax
             addq $$4,    %rdi
             loop         loop${:uid}

             reg0${:uid}:
             movq (%rsi), %r9
             addq $$8,    %rsi
             movl (%rdi), %eax
             movq %rax,   %r8
             addq $$4,    %rdi

             reg1${:uid}:
             movq (%rsi), %rcx
             addq $$8,    %rsi
             movl (%rdi), %edx
             addq $$4,    %rdi

             reg2${:uid}:
             movq (%rsi), %rsi
             movl (%rdi), %eax
             movq %rax,   %rdi

             callq        *$5
             movq %rax,   $0

             shl $$3,     %rbx
             addq %rbx,   %rsp
             jmp          exit${:uid}

             null_r8${:uid}:
             xorq %r8,    %r8
             jmp          reg1${:uid}

             null_rdx${:uid}:
             xorq %rdx,   %rdx
             jmp          reg2${:uid}

             exit${:uid}:"

             : "=r"(grid)
             : "m"(types.as_ptr()), "m"(values.as_ptr()),
               "m"(len), "i"(NEWT_GRID_EMPTY), "r"(func)
             : "rsp", "rax", "rbx", "rcx", "rdi", "rsi", "r8", "r9"
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
        asm! {
            "movq $3,     %rcx
             movq $2,     %rsi
             movq $1,     %rdi
             addq $$8,    %rdi

             subq $$8,    %rsp
             xorq %rax,   %rax
             pushq        %rax
             xorq %rbx,   %rbx
             addq $$2,    %rbx

             sub $$3,     %rcx
             jz           reg0${:uid}
             cmp $$-1,    %rcx
             je           null_r8${:uid}
             cmp $$-2,    %rcx
             je           null_rdx${:uid}
             movq %rcx,   %rax
             shl $$1,     %rax
             addq %rax,   %rbx

             loop${:uid}:
             movq %rsi,   %rax
             pushq        %rax
             addq $$8,    %rsi
             movq (%rdi), %rax
             pushq        %rax
             addq $$8,    %rdi
             loop         loop${:uid}

             reg0${:uid}:
             movq %rsi,   %r9
             addq $$8,    %rsi
             movq (%rdi), %r8
             addq $$8,    %rdi

             reg1${:uid}:
             movq %rsi,   %rcx
             addq $$8,    %rsi
             movq (%rdi), %rdx
             addq $$8,    %rdi

             reg2${:uid}:
             movq (%rdi), %rdi

             call newtButtonBar
             movq %rax,   $0

             shl $$3,     %rbx
             addq %rbx,   %rsp
             jmp          exit${:uid}

             null_r8${:uid}:
             xorq %r8,    %r8
             jmp          reg1${:uid}

             null_rdx${:uid}:
             xorq %rdx,   %rdx
             jmp          reg2${:uid}

             exit${:uid}:"

            : "=r"(grid)
            : "m"(button_ptrs.as_ptr()), "m"(buf), "m"(buttons.len())
            : "rsp", "rax", "rbx", "rcx", "rdi", "rsi", "r8", "r9"
        }
        grid
    }
}

///
/// Open a window containing a `Listbox` menu.
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
        asm! {
            "movq $10,    %rcx
             movq $9,     %rsi
             movq %rcx,   %rbx

             test $$1,    %rcx
             jz           loop${:uid}

             subq $$8,    %rsp
             addq $$1,    %rbx

             loop${:uid}:
             movq (%rsi), %rax
             pushq        %rax
             addq $$8,    %rsi
             loop         loop${:uid}

             movq $8,     %rax
             pushq        %rax
             movq $7,     %rax
             pushq        %rax

             movq $1,     %rdi
             movq $2,     %rsi
             mov  $3,     %rdx
             mov  $4,     %rcx
             mov  $5,     %r8
             mov  $6,     %r9

             call newtWinMenu
             mov  %eax,   $0

             addq $$2,    %rbx
             shl  $$3,    %rbx
             addq %rbx,   %rsp"

            : "=r"(rv)
            : "m"(title.as_ptr()), "m"(text.as_ptr()), "m"(suggested_width),
              "m"(flex_down), "m"(flex_up), "m"(max_list_height),
              "m"(item_ptrs.as_ptr()), "m"(&list_item),
              "m"(button_ptrs.as_ptr()), "m"(button_ptrs.len())
            : "rsp", "rax", "rbx", "rcx", "rdx", "rdi", "rsi", "r8", "r9"
        }
    }

    (rv, list_item)
}

///
/// Open a window containing a number of text `Entry`s.
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
/// * `entries` - A slice containing of `WinEntry`s providing
///               initial settings for each `Entry` field.
/// * `buttons` - A slice containing the text for a number of buttons to
///               display in the window.
///
/// Returns the number of the button pressed to close the window.
///
/// Each `WinEntry` in the `entries` array will be modified to contain the
/// data entered by the user. This can be accessed via the
/// [`WinEntry.value()`][win_entry_value] function.
///
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

        asm! {
            "movq $9,     %rcx
             movq $8,     %rsi
             movq %rcx,   %rbx

             test $$1,    %rcx
             jnz          loop${:uid}

             subq $$8,    %rsp
             addq $$1,    %rbx

             loop${:uid}:
             movq (%rsi), %rax
             pushq        %rax
             addq $$8,    %rsi
             loop         loop${:uid}

             movq $7,     %rax
             pushq        %rax

             movq $1,     %rdi
             movq $2,     %rsi
             mov  $3,     %rdx
             mov  $4,     %rcx
             mov  $5,     %r8
             mov  $6,     %r9

             call newtWinEntries
             mov  %eax,   $0

             addq $$1,    %rbx
             shl  $$3,    %rbx
             addq %rbx,   %rsp"

            : "=r"(rv)
            : "m"(title.as_ptr()), "m"(text.as_ptr()), "m"(suggested_width),
              "m"(flex_down), "m"(flex_up), "m"(data_width), "m"(entries_buf),
              "m"(button_ptrs.as_ptr()),  "m"(button_ptrs.len())
            : "rsp", "rax", "rbx", "rcx", "rdx", "rdi", "rsi", "r8", "r9"
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
