use syn::Path;

use quote::{format_ident, quote};
use rustic_reflect::*;

pub fn gen_callback_method(client_handle: &Path, method: &TraitMethodSignature, parameters: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
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
                TraitMethodParameterType::Type(_) => quote! {
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
    let return_type_conversion = super::convert_return_type(&method.return_type, true);
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
    } else {
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
        TraitMethodReturnType::Vec(_) => quote! { *mut c_void },
    }
}
