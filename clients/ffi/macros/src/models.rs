use proc_macro2::TokenStream;

use quote::{format_ident, quote};
use rustic_reflect::*;

pub fn gen_ffi_model(struct_decl: StructSignature) -> TokenStream {
    let original = format_ident!("{}", struct_decl.name);
    let name = format_ident!("FFI{}", struct_decl.name);

    let fields: TokenStream = struct_decl
        .fields
        .iter()
        .map(|(name, decl)| {
            let name = format_ident!("{}", name);
            let ty = to_type(&decl.ty);
            quote! {
                pub #name: #ty,
            }
        })
        .collect();

    let conversions: TokenStream = struct_decl
        .fields
        .iter()
        .map(|(name, decl)| field_conversion(name, decl))
        .collect();

    quote! {
        #[derive(Debug, Clone)]
        #[repr(C)]
        pub struct #name {
            #fields
        }

        impl From<rustic_api::models::#original> for #name {
            fn from(model: rustic_api::models::#original) -> #name {
                #name {
                    #conversions
                }
            }
        }
    }
}

fn to_type(ty: &StructFieldType) -> TokenStream {
    match &ty {
        StructFieldType::Type(ty) if ty == "String" => quote! { *const libc::c_char },
        StructFieldType::Type(ty) if ty == "bool" => quote! { bool },
        StructFieldType::Type(ty) if ty == "u64" => quote! { libc::c_ulong },
        StructFieldType::Type(ty) if ty == "f32" => quote! { libc::c_float },
        StructFieldType::Type(ty) if ty == "f64" => quote! { libc::c_double },
        StructFieldType::Type(ty) if ty == "HashMap < String, MetaValueModel >" => quote! { ::std::collections::HashMap<String, FFIMetaValueModel> },
        StructFieldType::Type(ty) => {
            let ty = format_ident!("FFI{}", ty);
            quote! { *const #ty }
        }
        StructFieldType::Option(nested) => to_type(&nested),
        _ => quote! { *const libc::c_void },
    }
}

fn field_conversion(name: &str, decl: &StructField) -> TokenStream {
    let name = format_ident!("{}", name);
    match &decl.ty {
        StructFieldType::Type(ty) if ty == "String" => quote! { #name: cstr!(model.#name), },
        StructFieldType::Type(ty) if ty == "bool" || ty == "u64" || ty == "f32" || ty == "f64" => {
            quote! { #name: model.#name, }
        },
        StructFieldType::Type(ty) if ty == "HashMap < String, MetaValueModel >" => quote! { #name: model.#name.into_iter().map(|(k, v)| (k, v.into())).collect(), },
        StructFieldType::Option(nested) => {
            match nested.as_ref() {
                StructFieldType::Type(ref ty) if ty == "bool" => {
                    // TODO: this is not quite the same but close enough for now
                    quote! { #name: model.#name.unwrap_or_default(), }
                }
                StructFieldType::Type(ref ty) if ty == "String" => {
                    quote! { #name: optional_cstr!(model.#name), }
                }
                // TODO: maybe we should add a layer of indirection (aka a pointer) here, as this may cause problems
                StructFieldType::Type(ref ty) if ty == "u64" || ty == "f32" || ty == "f64" => {
                    quote! { #name: optional_number!(model.#name), }
                }
                StructFieldType::Type(ref ty) => {
                    let ty = format_ident!("FFI{}", ty);
                    quote! { #name: nested_optional!(model.#name, #ty), }
                }
                _ => quote! { #name: ::std::ptr::null(), },
            }
        }
        _ => quote! { #name: ::std::ptr::null(), },
    }
}
