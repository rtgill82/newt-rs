use std::ffi::CString;
use newt_sys::*;

#[cfg(feature = "asm")]
use crate::intern::funcs::*;

pub fn win_message(title: &str, button_text: &str, text: &str) {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button = CString::new(button_text).unwrap();
        let c_text = CString::new(text).unwrap();

        newtWinMessage(c_title.as_ptr() as *mut i8,
                       c_button.as_ptr() as *mut i8,
                       c_text.as_ptr() as *mut i8);
    }
}

pub fn win_choice(title: &str, button1: &str, button2: &str, text: &str)
  -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let c_text = CString::new(text).unwrap();

        newtWinChoice(c_title.as_ptr() as *mut i8,
                      c_button1.as_ptr() as *mut i8,
                      c_button2.as_ptr() as *mut i8,
                      c_text.as_ptr() as *mut i8) as i32
    }
}

pub fn win_ternary(title: &str, button1: &str, button2: &str, button3: &str,
                   text: &str) -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let c_button3 = CString::new(button3).unwrap();
        let c_text = CString::new(text).unwrap();

        newtWinTernary(c_title.as_ptr() as *mut i8,
                       c_button1.as_ptr() as *mut i8,
                       c_button2.as_ptr() as *mut i8,
                       c_button3.as_ptr() as *mut i8,
                       c_text.as_ptr() as *mut i8) as i32
    }
}

#[cfg(feature = "asm")]
#[cfg(target_arch = "x86_64")]
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

             movq $$0,    %rax
             call newtWinMenu
             mov  %eax,   $0

             addq $$2,    %rbx
             movq %rbx,   %rax
             movq $$8,    %rbx
             mulq %rbx
             addq %rax,   %rsp"

            : "=r"(rv)
            : "m"(title.as_ptr()), "m"(text.as_ptr()), "m"(suggested_width),
              "m"(flex_down), "m"(flex_up), "m"(max_list_height),
              "m"(item_ptrs.as_ptr()), "m"(&list_item),
              "m"(button_ptrs.as_ptr()), "m"(button_ptrs.len())
            : "rsp", "rax", "rbx", "rcx", "rdx", "rdi", "rsi", "r8", "r9"
        }
    }

    return (rv, list_item);
}

#[cfg(feature = "asm")]
#[cfg(target_arch = "x86")]
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

             mov $$0,    %eax
             call newtWinMenu
             mov %eax,   $0

             add $$8,    %ebx
             mov %ebx,   %eax
             mov $$4,    %ebx
             mul %ebx
             add %eax,   %esp"

            : "=r"(rv)
            : "m"(title.as_ptr()), "m"(text.as_ptr()), "m"(suggested_width),
              "m"(flex_down), "m"(flex_up), "m"(max_list_height),
              "m"(item_ptrs.as_ptr()), "m"(&list_item),
              "m"(button_ptrs.as_ptr()), "m"(button_ptrs.len())
            : "esp", "eax", "ebx", "ecx", "edx", "esi"
        }
    }

    return (rv, list_item);
}
