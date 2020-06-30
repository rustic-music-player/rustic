use std::collections::HashMap;
use std::sync::RwLock;

use syn::{ItemStruct, Type};

use crate::helpers::unwrap_generic;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

lazy_static! {
    static ref STRUCTS: RwLock<Vec<StructSignature>> = RwLock::new(Vec::new());
}

pub fn put_struct(item_struct: ItemStruct) {
    let ident = item_struct.ident;

    let fields: HashMap<_, _> = item_struct
        .fields
        .into_iter()
        .filter(|item| item.ident.is_some())
        .map(|item| {
            let name = item.ident.unwrap().to_string();

            let field = StructField { ty: item.ty.into() };

            (name, field)
        })
        .collect();

    let sig = StructSignature {
        name: ident.to_string(),
        fields,
    };
    let mut structs = STRUCTS.write().unwrap();
    structs.push(sig);
}

pub fn get_structs() -> Vec<StructSignature> {
    let structs = STRUCTS.read().unwrap();
    structs.iter().cloned().collect()
}

#[derive(Debug, Clone)]
pub struct StructSignature {
    pub name: String,
    pub fields: HashMap<String, StructField>,
}

impl ToTokens for StructSignature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let fields: TokenStream = self
            .fields
            .iter()
            .map(|(name, field)| {
                quote! { fields.insert(#name.into(), #field); }
            })
            .collect();
        tokens.extend(quote! {
            {
                let mut fields = ::std::collections::HashMap::new();
                #fields
                rustic_reflect::StructSignature {
                    name: #name.into(),
                    fields
                }
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub ty: StructFieldType,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = &self.ty;
        tokens.extend(quote! { rustic_reflect::StructField { ty: #ty }})
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructFieldType {
    Option(Box<StructFieldType>),
    Vec(Box<StructFieldType>),
    Type(String),
}

impl From<Type> for StructFieldType {
    fn from(path: Type) -> Self {
        match path {
            Type::Tuple(_) => unimplemented!("From<Type> for TraitMethodReturnType Tuple"),
            Type::Path(ref type_path) => {
                let segment = type_path.path.segments.first().unwrap();
                let ident = segment.ident.to_string();
                if ident == "Vec" {
                    let p = unwrap_generic(type_path);
                    StructFieldType::Vec(Box::new(StructFieldType::from(p)))
                } else if ident == "Option" {
                    let p = unwrap_generic(type_path);
                    StructFieldType::Option(Box::new(StructFieldType::from(p)))
                } else {
                    let p = quote! { #path };
                    StructFieldType::Type(p.to_string())
                }
            }
            _ => {
                let p = quote! { #path };
                StructFieldType::Type(p.to_string())
            }
        }
    }
}

impl ToTokens for StructFieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = match self {
            StructFieldType::Option(return_type) => {
                quote! { rustic_reflect::StructFieldType::Option(Box::new(#return_type)) }
            }
            StructFieldType::Type(ref p) => {
                quote! { rustic_reflect::StructFieldType::Type(String::from(#p)) }
            }
            StructFieldType::Vec(return_types) => {
                quote! { rustic_reflect::StructFieldType::Vec(Box::new(#return_types)) }
            }
        };
        tokens.extend(token);
    }
}
