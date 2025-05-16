//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
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
use syn::{Generics,Ident};

pub fn impl_component_common(name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ crate::intern::ComponentPtr for #name #type_
            #where_
        {
            fn ptr(&self) -> *mut ::std::os::raw::c_void {
                let ptr = self.co.get();
                if ptr.is_null() {
                    panic!("Component has already been destroyed!");
                }
                ptr as *mut ::std::os::raw::c_void
            }

            fn co_ptr(&self) -> ::newt_sys::newtComponent {
                self.ptr() as ::newt_sys::newtComponent
            }

            fn grid_ptr(&self) -> ::newt_sys::newtGrid {
                self.ptr() as newt_sys::newtGrid
            }
        }

        impl #impl_ crate::Component for #name #type_
            #where_
        {
            fn co(&self) -> ::newt_sys::newtComponent {
                use crate::intern::ComponentPtr;
                self.co_ptr()
            }
        }

        #[cfg(not(feature = "asm"))]
        impl #impl_ crate::asm::AsComponent for #name #type_ #where_ { }

        #[cfg(feature = "asm")]
        impl #impl_ crate::asm::AsComponent for #name #type_
            #where_
        {
            fn as_component(&self) -> Option<&crate::Component> {
                Some(self)
            }
        }
    };
    gen.into()
}
