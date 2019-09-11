extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput,Generics,Ident};

pub fn impl_component_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = generics_remove_defaults(&ast.generics);
    let mut tokens = impl_component_base(&name, &generics);
    tokens.extend(impl_component_drop(&name, &generics));
    tokens.extend(impl_component_partial_eq_trait(&name, &generics));
    tokens.extend(impl_component_partial_eq(&name, &generics));
    tokens
}

fn impl_component_base(name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ ::intern::ComponentPtr for #name #type_
            #where_
        {
            fn ptr(&self) -> *mut ::libc::c_void {
                let ptr = self.co.get();
                if ptr.is_null() {
                    panic!("Component has already been destroyed!");
                }
                ptr as *mut ::libc::c_void
            }

            fn co_ptr(&self) -> ::newt_sys::newtComponent {
                self.ptr() as ::newt_sys::newtComponent
            }

            fn grid_ptr(&self) -> ::newt_sys::newtGrid {
                self.ptr() as newt_sys::newtGrid
            }
        }

        impl #impl_ ::Component for #name #type_
            #where_
        {
            fn co(&self) -> ::newt_sys::newtComponent {
                use ::intern::ComponentPtr;
                self.co_ptr()
            }
        }

        #[cfg(not(feature = "asm"))]
        impl #impl_ ::intern::AsComponent for #name #type_ #where_ { }

        #[cfg(feature = "asm")]
        impl #impl_ ::intern::AsComponent for #name #type_
            #where_
        {
            fn as_component(&self) -> Option<&::Component> {
                Some(self)
            }
        }

        #[cfg(not(feature = "asm"))]
        impl #impl_ ::intern::AsGrid for #name #type_ #where_ { }

        #[cfg(feature = "asm")]
        impl #impl_ ::intern::AsGrid for #name #type_
            #where_
        {
            fn as_grid(&self) -> Option<&::grid::r#trait::Grid> {
                None
            }
        }

        impl #impl_ ::widgets::WidgetFns for #name #type_ { }

        impl #impl_ ::intern::Child for #name #type_
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

        impl #impl_ ::intern::GridElementType for #name #type_
            #where_
        {
            fn grid_element_type(&self) -> u32 {
                use constants::NEWT_GRID_COMPONENT;
                NEWT_GRID_COMPONENT
            }
        }

        impl #impl_ ::intern::Nullify for #name #type_
        {
            fn nullify(&self) {
                self.co.replace(std::ptr::null_mut());
            }
        }
    };
    gen.into()
}

fn impl_component_drop(name: &Ident, generics: &Generics) -> TokenStream {
    if name == "Form" {
        return TokenStream::new();
    }

    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ std::ops::Drop for #name #type_
            #where_
        {
            fn drop(&mut self) {
                use ::Component;
                if !self.added_to_parent.get() {
                    unsafe {
                        ::newt_sys::newtComponentDestroy(self.co());
                    }
                }
            }
        }
    };
    gen.into()
}

fn impl_component_partial_eq_trait(name: &Ident, generics: &Generics)
        -> TokenStream {
    let generics_rhs = generics_add_rhs(&generics);
    let (_impl, type_, _where) = generics.split_for_impl();
    let (impl_, _type, where_) = generics_rhs.split_for_impl();
    let gen = quote! {
        impl #impl_ std::cmp::PartialEq<Rhs> for #name #type_
            #where_
        {
            fn eq(&self, other: &Rhs) -> bool {
                use ::Component;
                if self.co().is_null() {
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
  -> TokenStream {
    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ std::cmp::PartialEq<Box<dyn (::Component)>> for #name #type_
            #where_
        {
            fn eq(&self, other: &Box<dyn (::Component)>) -> bool {
                use ::Component;
                if self.co().is_null() {
                    return false
                }
                self.co() == other.co()
            }
        }

        impl #impl_ std::cmp::PartialEq<::widgets::form::ExitReason> for #name #type_
            #where_
        {
            fn eq(&self, other: &::widgets::form::ExitReason) -> bool {
                other == self
            }
        }
    };
    gen.into()
}

fn generics_add_rhs(generics: &Generics) -> Generics {
    use syn::{GenericParam,parse_str};
    let mut generics = generics.clone();
    let rhs: GenericParam = parse_str("Rhs: ::Component").unwrap();
    generics.params.push(rhs);
    generics
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
