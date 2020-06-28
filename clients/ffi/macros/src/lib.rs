use proc_macro::*;

use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::{Ident, Path, Result, Token};

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
            tokens.extend(gen_callback_method(&client_handle, &method, &parameters));
            tokens.extend(gen_blocking_method(&client_handle, &method, &parameters));
        }

        TokenStream::from(tokens)
    } else {
        println!("No signature found for trait {}", input.client_trait);
        TokenStream::new()
    }
}

fn gen_blocking_method(client_handle: &Path, method: &TraitMethodSignature, parameters: &Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    let call_params: Vec<_> = method
        .parameters
        .iter()
        .map(|param| {
            let name = format_ident!("{}", param.name);
            match param.type_ident {
                TraitMethodParameterType::String => quote! {
                            to_str(#name).unwrap().unwrap()
                        },
                TraitMethodParameterType::Type(ref p_type)
                if p_type.starts_with("Option") =>
                    {
                        quote! {
                                None
                            }
                    }
                TraitMethodParameterType::Type(ref p_type) => quote! {
                            unimplemented!()
                        },
            }
        })
        .collect();
    let method_name = format_ident!("{}", method.name);
    let exposed_name = format_ident!("client_{}_blocking", method.name);
    let return_type = to_return_type(&method.return_type);
    let return_type_conversion = convert_return_type(&method.return_type, true);
    quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #exposed_name(ptr: *mut RusticClientHandle, #(#parameters),*) #return_type {
            let mut client_handle = #client_handle::from_ptr(ptr);
            let client = client_handle.get_client();
            let mut rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async {
                let res = client.#method_name(#(#call_params),*).await.unwrap();
                #return_type_conversion
            })
        }
    }
}

fn to_return_type(return_type: &TraitMethodReturnType) -> proc_macro2::TokenStream {
    match return_type {
        TraitMethodReturnType::Unit => quote! {},
        TraitMethodReturnType::Type(t) => {
            let name = format_ident!("FFI{}", t);
            quote! { -> *const #name }
        }
        TraitMethodReturnType::Option(t) => to_return_type(t),
        TraitMethodReturnType::Vec(t) => quote! { -> *mut c_void },
    }
}

fn gen_callback_method(client_handle: &Path, method: &TraitMethodSignature, parameters: &Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    let call_params: Vec<_> = method
        .parameters
        .iter()
        .map(|param| {
            let name = format_ident!("{}", param.name);
            match param.type_ident {
                TraitMethodParameterType::String => quote! {
                    let #name = to_str(#name).unwrap().unwrap();
                },
                TraitMethodParameterType::Type(ref p_type)
                if p_type.starts_with("Option") =>
                    {
                        quote! {
                            let #name = None;
                        }
                    }
                TraitMethodParameterType::Type(ref p_type) => quote! {
                    let #name = unimplemented!();
                },
            }
        })
        .collect();
    let param_names: Vec<_> = method
        .parameters
        .iter()
        .map(|param| {
            let name = format_ident!("{}", param.name);
            quote! { #name }
        })
        .collect();
    let method_name = format_ident!("{}", method.name);
    let exposed_name = format_ident!("client_{}_cb", method.name);
    let return_type = to_return_type_cb(&method.return_type);
    let return_type_conversion = convert_return_type(&method.return_type, true);
    let content = quote! {
        let mut client_handle = #client_handle::from_ptr(ptr);
        let client = ::std::sync::Arc::clone(client_handle.get_client());

        #(#call_params)*

        RUNTIME.spawn(async move {
            let res = client.#method_name(#(#param_names),*).await.unwrap();
            callback(std::ptr::null_mut(), { #return_type_conversion })
        });
    };
    if parameters.is_empty() {
        quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #exposed_name(ptr: *mut RusticClientHandle, callback: fn(*mut c_char, #return_type)) {
                #content
            }
        }
    }else {
        quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #exposed_name(ptr: *mut RusticClientHandle, #(#parameters),*, callback: fn(*mut c_char, #return_type)) {
                #content
            }
        }
    }
}

fn to_return_type_cb(return_type: &TraitMethodReturnType) -> proc_macro2::TokenStream {
    match return_type {
        TraitMethodReturnType::Unit => quote! { () },
        TraitMethodReturnType::Type(t) => {
            let name = format_ident!("FFI{}", t);
            quote! { *const #name }
        }
        TraitMethodReturnType::Option(t) => to_return_type_cb(t),
        TraitMethodReturnType::Vec(t) => quote! { *mut c_void },
    }
}

fn convert_return_type(
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
