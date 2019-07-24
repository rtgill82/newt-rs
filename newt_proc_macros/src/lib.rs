#![recursion_limit="128"]
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

mod component;
mod grid;

use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    component::impl_component_macro(&ast)
}

#[proc_macro_derive(Grid)]
pub fn grid_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    grid::impl_grid_macro(&ast)
}
