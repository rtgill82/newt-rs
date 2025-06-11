//
// Copyright (C) 2019,2025 Robert Gill <rtgill82@gmail.com>
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

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput,Generics,Ident};

use crate::common::*;

pub fn impl_grid_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let mut tokens = impl_component_common(&name, &generics);
    tokens.extend(impl_grid_base(&name, &generics));
    tokens.extend(impl_component_base(&name, &generics));
    tokens.extend(impl_grid_child(&name, &generics));
    tokens.extend(impl_grid_drop(&name, &generics));
    tokens.extend(impl_grid_parent(&name, &generics));
    tokens
}

fn impl_grid_base(name: &Ident, generics: &Generics)
    -> TokenStream
{
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ crate::grid::r#trait::Grid for #name #type_
            #where_
        { }

        impl #impl_ crate::grid::r#trait::GridFns for #name #type_
            #where_
        { }
    };
    gen.into()
}

fn impl_component_base(name: &Ident, generics: &Generics)
    -> TokenStream
{
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ crate::asm::AsGrid for #name #type_
            #where_
        {
            fn as_grid(&self) -> Option<&crate::grid::r#trait::Grid> {
                Some(self)
            }
        }

        impl #impl_ crate::private::GridElementType for #name #type_
            #where_
        {
            fn grid_element_type(&self) -> u32 {
                use crate::constants::NEWT_GRID_SUBGRID;
                NEWT_GRID_SUBGRID
            }
        }
    };
    gen.into()
}

fn impl_grid_child(name: &Ident, generics: &Generics)
    -> TokenStream
{
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ crate::private::Child for #name #type_
            #where_
        {
            fn add_to_parent(&self)
              -> Result<(), &'static str> {
                if self.added_to_parent.get() {
                    return Err("Grid already belongs to a parent.");
                }

                for child in self.children.iter() {
                    child.add_to_parent()?;
                }
                self.added_to_parent.set(true);
                Ok(())
            }

            fn added_to_parent(&self) -> bool {
                self.added_to_parent.get()
            }
        }
    };
    gen.into()
}

fn impl_grid_parent(name: &Ident, generics: &Generics)
    -> TokenStream
{
    if name == "ButtonBar" {
        return TokenStream::new();
    }

    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ crate::grid::Parent for #name #type_
            #where_
        {
            fn children(&self) -> Vec<&crate::Component> {
                use crate::grid::Parent;
                use crate::constants::NEWT_GRID_COMPONENT;
                use crate::private::GridElementType;

                let mut vec: Vec<&crate::Component> = Vec::new();
                for child in self.children.iter() {
                    if let Some(grid) = child.as_grid() {
                        for child in grid.children().iter() {
                            vec.push(*child);
                        }
                    } else {
                        vec.push(*child);
                    }
                }
                vec
            }
        }
    };
    gen.into()
}

fn impl_grid_drop(name: &Ident, generics: &Generics)
    -> TokenStream
{
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ std::ops::Drop for #name #type_
            #where_
        {
            fn drop(&mut self) {
                use crate::private::ComponentPtr;
                unsafe { ::newt_sys::newtGridFree(self.grid_ptr(), 0); }
            }
        }
    };
    gen.into()
}
