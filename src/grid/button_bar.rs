extern crate newt_sys;
use libc::c_void;
use std::mem::size_of;
use newt_sys::*;

use crate::components::Button;
use crate::intern::funcs::*;

///
/// Creates a row of buttons.
///
#[derive(Grid)]
pub struct ButtonBar {
    grid: newtGrid,
    added_to_parent: bool,
    children: Option<Vec<Button>>
}

impl ButtonBar {
    #[cfg(target_arch = "x86_64")]
    ///
    /// Create a new grid containing a row of buttons. The buttons will
    /// be labeled with the strings provided in `buttons`.
    ///
    /// * `buttons` - A list of strings to use as button labels.
    ///
    pub fn new(buttons: &[&str]) -> ButtonBar {
        let mut grid: newtGrid;

        let buttons = str_slice_to_cstring_vec(buttons);
        let mut button_ptrs = cstring_vec_to_ptrs(&buttons);
        button_ptrs.reverse();

        let buttons_buf: *mut newtComponent;
        unsafe {
            let size = size_of::<newtComponent>() * (buttons.len());
            buttons_buf = libc::malloc(size) as *mut newtComponent;
            libc::memset(buttons_buf as *mut c_void, 0, size);

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
                 jz button_bar_reg0
                 cmp $$-1,    %rcx
                 je button_bar_null_r8
                 cmp $$-2,    %rcx
                 je button_bar_null_rdx
                 movq %rcx,   %rax
                 shl $$1,     %rax
                 addq %rax,   %rbx

                 button_bar_loop:
                 movq %rsi,   %rax
                 pushq        %rax
                 addq $$8,    %rsi
                 movq (%rdi), %rax
                 pushq        %rax
                 addq $$8,    %rdi
                 loop button_bar_loop

                 button_bar_reg0:
                 movq %rsi,   %r9
                 addq $$8,    %rsi
                 movq (%rdi), %r8
                 addq $$8,    %rdi

                 button_bar_reg1:
                 movq %rsi,   %rcx
                 addq $$8,    %rsi
                 movq (%rdi), %rdx
                 addq $$8,    %rdi

                 button_bar_reg2:
                 movq (%rdi), %rdi

                 xorq %rax,   %rax
                 call newtButtonBar
                 movq %rax,   $0

                 shl $$3,     %rbx
                 addq %rbx,   %rsp
                 jmp button_bar_exit

                 button_bar_null_r8:
                 xorq %r8,    %r8
                 jmp button_bar_reg1

                 button_bar_null_rdx:
                 xorq %rdx,   %rdx
                 jmp button_bar_reg2

                 button_bar_exit:"

                : "=r"(grid)
                : "m"(button_ptrs.as_ptr()), "m"(buttons_buf),
                  "m"(buttons.len())
                : "rsp", "rax", "rbx", "rcx", "rdi", "rsi", "r8", "r9"
            }

            let num_buttons = buttons.len();
            let mut buttons = Vec::new();
            let mut button_co = *buttons_buf.add(num_buttons - 1);
            buttons.push(Button::new_co(button_co));
            for i in (0..num_buttons - 1).rev() {
                button_co = *buttons_buf.add(i);
                buttons.push(Button::new_co(button_co));
            }

            libc::free(buttons_buf as *mut c_void);
            ButtonBar {
                grid,
                added_to_parent: false,
                children: Some(buttons)
            }
        }
    }

    #[cfg(target_arch = "x86")]
    ///
    /// Create a new grid containing a row of buttons. The buttons will
    /// be labeled with the strings provided in `buttons`.
    ///
    /// * `buttons` - A list of strings to use as button labels.
    ///
    pub fn new(buttons: &[&str]) -> ButtonBar {
        let mut grid: newtGrid;

        let buttons = str_slice_to_cstring_vec(buttons);
        let mut button_ptrs = cstring_vec_to_ptrs(&buttons);
        button_ptrs.reverse();

        let buttons_buf: *mut newtComponent;
        unsafe {
            let size = size_of::<newtComponent>() * (buttons.len());
            buttons_buf = libc::malloc(size) as *mut newtComponent;
            libc::memset(buttons_buf as *mut c_void, 0, size);

            asm! {
                "mov $3,      %ecx
                 mov $2,      %esi
                 mov $1,      %edi
                 add $$4,     %edi

                 sub $$4,     %esp
                 xor %eax,    %eax
                 push         %eax
                 mov %ecx,    %ebx
                 add $$1,     %ebx

                 button_bar_loop:
                 mov %esi,    %eax
                 push         %eax
                 add $$4,     %esi
                 mov (%edi),  %eax
                 push         %eax
                 add $$4,     %edi
                 loop button_bar_loop

                 xor %eax,    %eax
                 call newtButtonBar
                 mov %eax,    $0

                 shl $$3,     %ebx
                 add %ebx,    %esp"

                : "=r"(grid)
                : "m"(button_ptrs.as_ptr()), "m"(buttons_buf),
                  "m"(buttons.len())
                : "esp", "eax", "ebx", "ecx", "edi", "esi"
            }

            let num_buttons = buttons.len();
            let mut buttons = Vec::new();
            let mut button_co = *buttons_buf.add(num_buttons - 1);
            buttons.push(Button::new_co(button_co));
            for i in (0..num_buttons - 1).rev() {
                button_co = *buttons_buf.add(i);
                buttons.push(Button::new_co(button_co));
            }

            libc::free(buttons_buf as *mut c_void);
            ButtonBar {
                grid,
                added_to_parent: false,
                children: Some(buttons)
            }
        }
    }

    ///
    /// Return the array of buttons contained by the grid.
    ///
    pub fn buttons(&self) -> &[Button] {
        if let Some(buttons) = &self.children {
            return buttons.as_slice();
        }
        unreachable!();
    }
}
