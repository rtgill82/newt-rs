extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput,Generics,Ident};

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_component_macro(&ast)
}

fn impl_component_macro(ast: &DeriveInput) -> TokenStream {
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
        impl #impl_ ::components::Component for #name #type_
            #where_
        {
            fn co(&self) -> ::newt_sys::newtComponent {
                self.co
            }

            fn attach_to_form(&mut self) {
                self.attached_to_form = true;
            }

            fn attached_to_form(&self) -> bool {
                self.attached_to_form
            }
        }
    };
    gen.into()
}

fn impl_component_drop(name: &Ident, generics: &Generics) -> TokenStream {
    if name == "Form" || name == "VerticalScrollbar" {
        return TokenStream::new();
    }

    let (impl_, type_, where_) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_ std::ops::Drop for #name #type_
            #where_
        {
            fn drop(&mut self) {
                if !self.attached_to_form {
                    unsafe {
                        ::newt_sys::newtComponentDestroy(self.co);
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
                if self.co == std::ptr::null_mut() {
                    return false
                }
                self.co == other.co()
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
        impl #impl_ std::cmp::PartialEq<Box<dyn (::components::Component)>> for #name #type_
            #where_
        {
            fn eq(&self, other: &Box<dyn (::components::Component)>) -> bool {
                if self.co == std::ptr::null_mut() {
                    return false
                }
                self.co == other.co()
            }
        }

        impl #impl_ std::cmp::PartialEq<::components::form::ExitReason> for #name #type_
            #where_
        {
            fn eq(&self, other: &::components::form::ExitReason) -> bool {
                other == self
            }
        }
    };
    gen.into()
}

fn generics_add_rhs(generics: &Generics) -> Generics {
    use syn::{GenericParam,parse_str};

    let mut generics = generics.clone();
    let rhs: GenericParam = parse_str("Rhs: ::components::Component").unwrap();
    generics.params.push(rhs);
    generics
}

fn generics_remove_defaults(generics: &Generics) -> Generics {
    use syn::GenericParam::Type;

    let mut generics = generics.clone();
    for mut param in generics.params.iter_mut() {
        if let Type(ref mut type_) = param {
            type_.default = None;
        }
    }
    generics
}
