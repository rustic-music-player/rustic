use proc_macro::*;

use syn::{Ident, Path, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

use quote::{format_ident, quote};
use rustic_api::get_signature_for_trait;
use rustic_reflect::*;

struct FFIClientWrapper {
    client_trait: Ident,
    client_handle: Path,
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

#[proc_macro]
pub fn ffi_client(args: TokenStream) -> TokenStream {
    let input = parse_macro_input!(args as FFIClientWrapper);

    if let Some(methods) = get_signature_for_trait(input.client_trait.to_string()) {
        let methods: Vec<TraitMethodSignature> = methods;
        let client_handle = input.client_handle;

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
                            #name: *const c_char
                        },
                        TraitMethodParameterType::Type(ref p_type) => {
                            quote! { #name: () }
                        }
                    }
                })
                .collect();
            tokens.extend(self::callback::gen_callback_method(&client_handle, &method, &parameters));
            tokens.extend(self::blocking::gen_blocking_method(&client_handle, &method, &parameters));
        }

        TokenStream::from(tokens)
    } else {
        println!("No signature found for trait {}", input.client_trait);
        TokenStream::new()
    }
}

mod blocking;

mod callback;

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
                    None => ptr::null()
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
                Box::into_raw(Box::new(res)) as *mut c_void
            }
        }
    }
}
