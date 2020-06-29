use syn::parse::{Parse, ParseStream};
use syn::{Ident, Path, Result, Token};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustic_reflect::*;

mod blocking;
mod callback;

pub struct FFIClientWrapper {
    pub client_trait: Ident,
    pub client_handle: Path,
}

pub fn gen_apis(input: &FFIClientWrapper, methods: Vec<TraitMethodSignature>) -> TokenStream {
    let mut tokens = quote! {};

    for method in methods {
        if !method.is_async {
            // TODO: handle streams later
            continue;
        }
        let parameters: Vec<_> = method
            .parameters
            .iter()
            .map(|param| {
                let name = format_ident!("{}", param.name);
                match param.type_ident {
                    TraitMethodParameterType::String => quote! {
                        #name: *const libc::c_char
                    },
                    TraitMethodParameterType::Type(_) => {
                        quote! { #name: () }
                    }
                }
            })
            .collect();
        tokens.extend(self::callback::gen_callback_method(
            &input.client_handle,
            &method,
            &parameters,
        ));
        tokens.extend(self::blocking::gen_blocking_method(
            &input.client_handle,
            &method,
            &parameters,
        ));
    }
    tokens
}

impl Parse for FFIClientWrapper {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let handle_ident = input.parse()?;
        Ok(FFIClientWrapper {
            client_trait: trait_ident,
            client_handle: handle_ident,
        })
    }
}

pub(crate) fn convert_return_type(
    return_type: &TraitMethodReturnType,
    as_ptr: bool,
) -> proc_macro2::TokenStream {
    match return_type {
        TraitMethodReturnType::Unit => quote! { () },
        TraitMethodReturnType::Type(t) => {
            let name = format_ident!("FFI{}", t);
            if as_ptr {
                quote! {
                    let res: #name = res.into();
                    Box::into_raw(Box::new(res))
                }
            } else {
                quote! {
                    let res: #name = res.into();
                    res
                }
            }
        }
        TraitMethodReturnType::Option(t) => {
            let return_type = convert_return_type(t, true);
            quote! {
                match res {
                    Some(res) => {
                        #return_type
                    },
                    None => ::std::ptr::null()
                }
            }
        }
        TraitMethodReturnType::Vec(t) => {
            let return_type = convert_return_type(t, false);
            quote! {
                let res: Vec<_> = res
                    .into_iter()
                    .map(|res| {
                        #return_type
                    })
                    .collect();
                Box::into_raw(Box::new(res)) as *mut libc::c_void
            }
        }
    }
}
