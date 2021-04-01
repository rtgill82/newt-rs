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

#![recursion_limit="256"]
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
