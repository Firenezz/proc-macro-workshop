use proc_macro2::TokenStream;
use syn::{Ident, Type};

use crate::util;



pub fn emit_setter(name: &Ident, ty: &Type, optional: bool) -> TokenStream {
    if optional {
        // First we need to get the optional path segment "Option"
        let non_optional_type = util::get_non_optional_type(ty);

        quote::quote! {
            fn #name(&mut self, value: #non_optional_type) -> &mut Self {
                self.#name = Some(value);
                self
            }
        }
    } else {
        quote::quote! {
            fn #name(&mut self, value: #ty) -> &mut Self {
                self.#name = Some(value);
                self
            }
        }
    }
}

pub fn emit_build_field(name: &Ident, optional: bool) -> TokenStream {
    if optional {
        quote::quote! {
            #name: self.#name.take()
        }
    } else {
        quote::quote! {
            #name: self.#name.take().ok_or(format!("field {} is required", stringify!(#name)))?
        }
    }
    
}