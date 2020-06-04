use rustic_reflect::*;
use syn::{parse_macro_input, ItemTrait};

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro_attribute]
pub fn reflect(_: TokenStream, input: TokenStream) -> TokenStream {
    let trait_input = input.clone();
    let trait_item = parse_macro_input!(trait_input as ItemTrait);

    put_trait(trait_item);

    input
}

#[proc_macro]
pub fn export_reflections(_: TokenStream) -> TokenStream {
    let traits = get_traits();

    let trait_blocks: proc_macro2::TokenStream = traits
        .into_iter()
        .map(|(ident, signature)| {
            quote! {
                if ident == #ident {
                    return Some({ #signature })
                }
            }
        })
        .collect();

    let result = quote! {
        #[doc(hidden)]
        #[inline]
        pub fn get_signature_for_trait(ident: String) -> Option<Vec<rustic_reflect::TraitMethodSignature>> {
            #trait_blocks

            None
        }
    };
    result.into()
}
