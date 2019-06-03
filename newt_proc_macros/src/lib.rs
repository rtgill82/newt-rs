#![recursion_limit="128"]
extern crate proc_macro;
extern crate syn;

use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::token::Comma;

#[macro_use]
extern crate quote;

mod component;
mod grid;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    component::impl_component_macro(&ast)
}

#[proc_macro_derive(ComponentFuncs)]
pub fn component_funcs_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        impl ::components::component::ComponentFuncs for #name { }
    };
    gen.into()
}

#[proc_macro_derive(Grid)]
pub fn grid_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    grid::impl_grid_macro(&ast)
}

#[proc_macro]
pub fn grid_asm_x86_64(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Comma>::parse_separated_nonempty;
    let args = parser.parse(input).unwrap();

    if args.len() != 5 {
        panic!("Invalid number of arguments.");
    }

    let mut args_iter = args.iter();

    let func;
    if let Expr::Path(expr) = args_iter.next().unwrap() {
        func = expr.path.segments.last().unwrap()
                   .value().ident.to_string();
    } else {
        panic!();
    }

    let types = args_iter.next().unwrap();
    let values = args_iter.next().unwrap();
    let length = args_iter.next().unwrap();
    let rv = args_iter.next().unwrap();

    let asm = format!(
        "movq $3,     %rcx
         movq $2,     %rsi
         movq $1,     %rdi

         subq $$8,    %rsp
         movq $4,     %rax
         pushq        %rax
         xorq %rbx,   %rbx
         addq $$2,    %rbx

         sub $$3,     %rcx
         jz {f}_reg0
         cmp $$-1,    %rcx
         je {f}_null_r8
         cmp $$-2,    %rcx
         je {f}_null_rdx
         movq %rcx,   %rax
         shl $$1,     %rax
         addq %rax,   %rbx

         {f}_loop:
         movq (%rsi), %rax
         pushq        %rax
         addq $$8,    %rsi
         movl (%rdi), %eax
         pushq        %rax
         addq $$4,    %rdi
         loop {f}_loop

         {f}_reg0:
         movq (%rsi), %r9
         addq $$8,    %rsi
         movl (%rdi), %eax
         movq %rax,   %r8
         addq $$4,    %rdi

         {f}_reg1:
         movq (%rsi), %rcx
         addq $$8,    %rsi
         movl (%rdi), %edx
         addq $$4,    %rdi

         {f}_reg2:
         movq (%rsi), %rsi
         movl (%rdi), %eax
         movq %rax,   %rdi

         xorq %rax,   %rax
         call {f}
         movq %rax,   $0

         shl $$3,     %rbx
         addq %rbx,   %rsp
         jmp {f}_exit

         {f}_null_r8:
         movq $$0,    %r8
         jmp {f}_reg1

         {f}_null_rdx:
         movq $$0,    %rdx
         jmp {f}_reg2

         {f}_exit:", f = func);

    let gen = quote! {
        unsafe {
            asm! {
                #asm

                : "=r"(#rv)
                : "m"(#types.as_ptr()), "m"(#values.as_ptr()),
                  "m"(#length), "i"(NEWT_GRID_EMPTY)
                : "rsp", "rax", "rbx", "rcx", "rdi", "rsi", "r8", "r9"
            }
        }
    };
    gen.into()
}
