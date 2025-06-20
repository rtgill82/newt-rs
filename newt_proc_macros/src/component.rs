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

pub fn impl_component_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = generics_remove_defaults(&ast.generics);
    let mut tokens = impl_component_common(&name, &generics);
    tokens.extend(impl_component_base(&name, &generics));
    tokens.extend(impl_component_drop(&name, &generics));
    tokens.extend(impl_component_partial_eq_trait(&name, &generics));
    tokens.extend(impl_component_partial_eq(&name, &generics));
    tokens
}

fn impl_component_base(name: &Ident, generics: &Generics)
    -> TokenStream
{
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ crate::widgets::WidgetFns for #name #type_ { }

        impl #impl_ crate::private::traits::Child for #name #type_
            #where_
        {
            fn add_to_parent(&self)
              -> Result<(), &'static str> {
                if self.added_to_parent.get() {
                    return Err("Component already belongs to a Form.");
                }
                self.added_to_parent.set(true);
                Ok(())
            }

            fn added_to_parent(&self) -> bool {
                self.added_to_parent.get()
            }
        }

        #[cfg(not(feature = "asm"))]
        impl #impl_ crate::asm::AsGrid for #name #type_ #where_ { }

        #[cfg(feature = "asm")]
        impl #impl_ crate::asm::AsGrid for #name #type_
            #where_
        {
            fn as_grid(&self) -> Option<&crate::grid::traits::Grid> {
                None
            }
        }

        impl #impl_ crate::private::traits::GridElementType for #name #type_
            #where_
        {
            fn grid_element_type(&self) -> u32 {
                use crate::constants::NEWT_GRID_COMPONENT;
                NEWT_GRID_COMPONENT
            }
        }

        impl #impl_ crate::private::traits::Nullify for #name #type_
        {
            fn nullify(&self) {
                self.co.replace(std::ptr::null_mut());
            }
        }
    };
    gen.into()
}

fn impl_component_drop(name: &Ident, generics: &Generics)
    -> TokenStream
{
    if name == "Form" {
        return TokenStream::new();
    }

    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ std::ops::Drop for #name #type_
            #where_
        {
            fn drop(&mut self) {
                use crate::Component;
                unsafe {
                    if !self.added_to_parent.get() {
                        ::newt_sys::newtComponentDestroy(self.co());
                    }
                }
            }
        }
    };
    gen.into()
}

fn impl_component_partial_eq_trait(name: &Ident, generics: &Generics)
    -> TokenStream
{
    let generics_rhs = generics_add_rhs(&generics);
    let (_impl, type_, _where) = generics.split_for_impl();
    let (impl_, _type, where_) = generics_rhs.split_for_impl();
    let gen = quote! {
        impl #impl_ std::cmp::PartialEq<Rhs> for #name #type_
            #where_
        {
            fn eq(&self, other: &Rhs) -> bool {
                use crate::Component;
                use crate::private::traits::ComponentPtr;

                if self.is_null() {
                    return false
                }
                self.co() == other.co()
            }
        }
    };
    let rv = gen.into();
    return rv;
}

fn impl_component_partial_eq(name: &Ident, generics: &Generics)
    -> TokenStream
{
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ std::cmp::PartialEq<Box<dyn (crate::Component)>> for #name #type_
            #where_
        {
            fn eq(&self, other: &Box<dyn (crate::Component)>) -> bool {
                use crate::Component;
                use crate::private::traits::ComponentPtr;

                if self.is_null() {
                    return false
                }
                self.co() == other.co()
            }
        }

        impl #impl_ std::cmp::PartialEq<crate::form::ExitReason> for #name #type_
            #where_
        {
            fn eq(&self, other: &crate::form::ExitReason) -> bool {
                other == self
            }
        }
    };
    gen.into()
}

fn generics_add_rhs(generics: &Generics) -> Generics {
    use syn::{GenericParam,parse_str};
    let mut generics = generics.clone();
    let rhs: GenericParam = parse_str("Rhs: crate::Component").unwrap();
    generics.params.push(rhs);
    generics
}

//
// Remove the default type as specified for generics. Specifying the default
// type for generics is not necessary for traits and I think including it
// used to cause compilation errors, but it doesn't seem to be an issue
// anymore. Leaving this function in place anyways.
//
fn generics_remove_defaults(generics: &Generics) -> Generics {
    use syn::GenericParam::Type;

    let mut generics = generics.clone();
    for param in generics.params.iter_mut() {
        if let Type(ref mut type_) = param {
            type_.default = None;
        }
    }
    generics
}
