use syn::Path;

use quote::{format_ident, quote};
use rustic_reflect::*;

use super::convert_return_type;

pub fn gen_blocking_method(client_handle: &Path, method: &TraitMethodSignature, parameters: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
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
                if p_type.starts_with("Option") => {
                    quote! {
                        None
                    }
                }
                TraitMethodParameterType::Type(_) => quote! {
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
        TraitMethodReturnType::Vec(_) => quote! { -> *mut c_void },
    }
}
