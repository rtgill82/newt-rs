extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput,Generics,Ident};

pub fn impl_grid_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = generics_remove_defaults(&ast.generics);
    let mut tokens = impl_grid_base(&name, &generics);
    tokens.extend(impl_component_base(&name, &generics));
    tokens.extend(impl_grid_child(&name, &generics));
    tokens.extend(impl_grid_drop(&name, &generics));
    tokens
}

fn impl_grid_base(name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ ::grid::r#trait::Grid for #name #type_
            #where_
        { }
    };
    gen.into()
}

fn impl_component_base(name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ ::intern::ComponentPtr for #name #type_
            #where_
        {
            fn ptr(&self) -> *mut ::libc::c_void {
                self.grid as *mut ::libc::c_void
            }

            fn as_co(&self) -> ::newt_sys::newtComponent {
                self.grid as ::newt_sys::newtComponent
            }

            fn as_grid(&self) -> ::newt_sys::newtGrid {
                self.grid
            }
        }

        impl #impl_ ::Component for #name #type_
            #where_
        {
            fn co(&self) -> ::newt_sys::newtComponent {
                use crate::intern::ComponentPtr;
                self.as_co()
            }
        }

        impl #impl_ ::intern::GridElementType for #name #type_
            #where_
        {
            fn grid_element_type(&self) -> u32 {
                use constants::NEWT_GRID_SUBGRID;
                NEWT_GRID_SUBGRID
            }
        }
    };
    gen.into()
}

fn impl_grid_child(name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ crate::intern::Child for #name #type_
            #where_
        {
            fn add_to_parent(&mut self, grid: bool)
              -> Result<(), &'static str> {
                if self.added_to_parent {
                    return Err("Grid already belongs to a parent.");
                }

                if let Some(children) = &mut self.children {
                    for child in children.iter_mut() {
                        child.add_to_parent(false)?;
                    }
                }

                if !grid {
                    self.added_to_parent = true;
                }
                Ok(())
            }

            fn added_to_parent(&self) -> bool {
                self.added_to_parent
            }
        }
    };
    gen.into()
}

fn impl_grid_drop(name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ std::ops::Drop for #name #type_
            #where_
        {
            fn drop(&mut self) {
                if !self.added_to_parent {
                    unsafe {
                        ::newt_sys::newtGridFree(self.grid, 1);
                    }
                }
            }
        }
    };
    gen.into()
}

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
