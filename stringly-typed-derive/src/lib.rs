//! A custom derive for the [stringly-typed] crate.
//! 
//! [stringly-typed]: 

#![recursion_limit="256"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate synstructure;
#[macro_use]
extern crate quote;

use syn::{Ident, DeriveInput};
use quote::Tokens;
use synstructure::Structure;

decl_derive!([StringlyTyped] => stringly_typed);

const INVALID_FIELD_ERROR: &'static str = "StringlyTyped can only be derived on normal struct fields";

fn stringly_typed(s: Structure) -> quote::Tokens {
    let name = &s.ast().ident;
    let name_as_str = name.to_string();

    let field_names: Vec<Ident> = s.variants()[0].bindings().into_iter().map(|field| {
        field.ast().ident.expect(INVALID_FIELD_ERROR).clone()
    })
    .collect();

    let set_body = field_names.iter().map(|name| {
        let name_as_str = name.to_string();
        quote!(#name_as_str => self.#name.set(keys, value),)
    })
    .fold(Tokens::new(), |mut acc, elem| {acc.append_all(elem); acc});

    let get_body = field_names.iter().map(|name| {
        let name_as_str = name.to_string();
        quote!(#name_as_str => self.#name.get(keys),)
    })
    .fold(Tokens::new(), |mut acc, elem| {acc.append_all(elem); acc});

    let field_names2 = field_names.clone();
    let impl_set = quote! {
        fn set<K, S>(&mut self, keys: K, value: ::stringly_typed::Value) -> Result<(), ::stringly_typed::UpdateError>
        where K: IntoIterator<Item = S>,
            S: AsRef<str> 
        {
            let mut keys = keys.into_iter();

            let element = keys.next()
                .ok_or_else(|| ::stringly_typed::UpdateError::NotEnoughKeys)?;

            match element.as_ref() {
                #set_body
                _ => Err(::stringly_typed::UpdateError::UnknownField {
                    valid_fields: &[
                        #( stringify!(#field_names2) ),*
                    ]
                })
            }
        }
    };
    
    let impl_get = quote! {
        fn get<K, S>(&self, keys: K) -> Result<::stringly_typed::Value, ::stringly_typed::UpdateError>
        where K: IntoIterator<Item = S>,
            S: AsRef<str> 
        {
            let mut keys = keys.into_iter();

            let element = keys.next()
                .ok_or_else(|| ::stringly_typed::UpdateError::NotEnoughKeys)?;

            match element.as_ref() {
                #get_body
                _ => Err(::stringly_typed::UpdateError::UnknownField {
                    valid_fields: &[
                        #( stringify!(#field_names) ),*
                    ]
                })
            }
        }
    };

    let data_type = quote! {
        fn data_type(&self) -> &'static str {
            #name_as_str
        }
    };

    quote! {
        impl ::stringly_typed::StringlyTyped for #name {
            #impl_set

            #impl_get

            #data_type
        }
    }
}
