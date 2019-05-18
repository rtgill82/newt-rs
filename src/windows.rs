use std::ffi::CString;
use libc::c_char;
use newt_sys::*;

#[cfg(feature = "asm")]
use crate::intern::funcs::*;
#[cfg(feature = "asm")]
use libc::c_void;
#[cfg(feature = "asm")]
use std::ffi::CStr;
#[cfg(feature = "asm")]
use std::mem::size_of;

pub fn win_message(title: &str, button_text: &str, text: &str) {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button = CString::new(button_text).unwrap();
        let escaped = str::replace(text, "%", "%%");
        let c_text = CString::new(escaped).unwrap();

        newtWinMessage(c_title.as_ptr() as *mut c_char,
                       c_button.as_ptr() as *mut c_char,
                       c_text.as_ptr() as *mut c_char);
    }
}

pub fn win_choice(title: &str, button1: &str, button2: &str, text: &str)
  -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let escaped = str::replace(text, "%", "%%");
        let c_text = CString::new(escaped).unwrap();

        newtWinChoice(c_title.as_ptr() as *mut c_char,
                      c_button1.as_ptr() as *mut c_char,
                      c_button2.as_ptr() as *mut c_char,
                      c_text.as_ptr() as *mut c_char) as i32
    }
}

pub fn win_ternary(title: &str, button1: &str, button2: &str, button3: &str,
                   text: &str) -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let c_button3 = CString::new(button3).unwrap();
        let escaped = str::replace(text, "%", "%%");
        let c_text = CString::new(escaped).unwrap();

        newtWinTernary(c_title.as_ptr() as *mut c_char,
                       c_button1.as_ptr() as *mut c_char,
                       c_button2.as_ptr() as *mut c_char,
                       c_button3.as_ptr() as *mut c_char,
                       c_text.as_ptr() as *mut c_char) as i32
    }
}

#[cfg(feature = "asm")]
#[cfg(target_arch = "x86_64")]
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
             jz win_menu_loop

             subq $$8,    %rsp
             addq $$1,    %rbx

             win_menu_loop:
             movq (%rsi), %rax
             pushq        %rax
             addq $$8,    %rsi
             loop         win_menu_loop

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

             xorq %rax,   %rax
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

#[cfg(feature = "asm")]
#[cfg(target_arch = "x86")]
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
            "mov $10,    %ecx
             mov $9,     %esi
             mov %ecx,   %ebx

             test $$1,   %ecx
             jz win_menu_loop

             sub $$4,    %esp
             add $$1,    %ebx

             win_menu_loop:
             mov (%esi), %eax
             push        %eax
             add $$4,    %esi
             loop        win_menu_loop

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

             xorl %eax,  %eax
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

#[cfg(feature = "asm")]
#[derive(Default)]
pub struct WinEntry {
    text: String,
    value: String,
    flags: i32
}

#[cfg(feature = "asm")]
impl WinEntry {
    pub fn new(text: &str, value: &str, flags: i32) -> WinEntry {
        WinEntry {
            text: String::from(text),
            value: String::from(value),
            flags
        }
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

#[cfg(feature = "asm")]
#[cfg(target_arch = "x86_64")]
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
             jnz win_entries_loop

             subq $$8,    %rsp
             addq $$1,    %rbx

             win_entries_loop:
             movq (%rsi), %rax
             pushq        %rax
             addq $$8,    %rsi
             loop         win_entries_loop

             movq $7,     %rax
             pushq        %rax

             movq $1,     %rdi
             movq $2,     %rsi
             mov  $3,     %rdx
             mov  $4,     %rcx
             mov  $5,     %r8
             mov  $6,     %r9

             xorq %rax,   %rax
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

#[cfg(feature = "asm")]
#[cfg(target_arch = "x86")]
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
            "mov $9,     %ecx
             mov $8,     %esi
             mov %ecx,   %ebx

             test $$1,   %ecx
             jnz win_entries_loop

             sub $$4,    %esp
             add $$1,    %ebx

             win_entries_loop:
             mov (%esi), %eax
             push        %eax
             add  $$4,   %esi
             loop        win_entries_loop

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

             xorl %eax,  %eax
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
