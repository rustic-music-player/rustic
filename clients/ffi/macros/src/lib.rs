use proc_macro::*;

use syn::parse_macro_input;

use rustic_api::{get_signature_for_trait, get_structs};

use crate::traits::FFIClientWrapper;

mod models;
mod traits;

#[proc_macro]
pub fn ffi_client(args: TokenStream) -> TokenStream {
    let input = parse_macro_input!(args as FFIClientWrapper);

    if let Some(methods) = get_signature_for_trait(input.client_trait.to_string()) {
        let tokens = crate::traits::gen_apis(&input, methods);

        TokenStream::from(tokens)
    } else {
        println!("No signature found for trait {}", input.client_trait);
        TokenStream::new()
    }
}

#[proc_macro]
pub fn client_models(_: TokenStream) -> TokenStream {
    let structs = get_structs();

    let tokens: proc_macro2::TokenStream = structs
        .into_iter()
        .map(crate::models::gen_ffi_model)
        .collect();

    TokenStream::from(tokens)
}
