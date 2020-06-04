use std::collections::HashMap;
use std::sync::RwLock;

use proc_macro2::TokenStream;
use syn::{
    FnArg, GenericArgument, ItemTrait, Pat, PatType, PathArguments, ReturnType, TraitItem, Type,
    TypePath,
};

use lazy_static::lazy_static;
use quote::{quote, ToTokens};

lazy_static! {
    static ref TRAITS: RwLock<HashMap<String, TraitSignature>> = RwLock::new(HashMap::new());
}

pub fn put_trait(item_trait: ItemTrait) {
    let ident = item_trait.ident;
    let items: Vec<_> = item_trait
        .items
        .into_iter()
        .filter_map(|item| {
            if let TraitItem::Method(method) = item {
                Some(method)
            } else {
                None
            }
        })
        .map(|item| TraitMethodSignature {
            name: item.sig.ident.to_string(),
            parameters: item
                .sig
                .inputs
                .into_iter()
                .filter_map(|input| {
                    if let FnArg::Typed(pat_type) = input {
                        Some(pat_type)
                    } else {
                        None
                    }
                })
                .map(TraitMethodParameter::from)
                .collect(),
            return_type: item.sig.output.into(),
        })
        .collect();

    let mut traits = TRAITS.write().unwrap();
    traits.insert(ident.to_string(), TraitSignature(items));
}

pub fn get_traits() -> Vec<(String, TraitSignature)> {
    let traits = TRAITS.read().unwrap();
    traits
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

#[derive(Debug, Clone)]
pub struct TraitSignature(pub Vec<TraitMethodSignature>);

#[derive(Debug, Clone)]
pub struct TraitMethodSignature {
    pub name: String,
    pub parameters: Vec<TraitMethodParameter>,
    pub return_type: TraitMethodReturnType,
}

#[derive(Debug, Clone)]
pub struct TraitMethodParameter {
    pub name: String,
    pub type_ident: TraitMethodParameterType,
}

#[derive(Debug, Clone)]
pub enum TraitMethodParameterType {
    String,
    Type(String),
}

impl From<PatType> for TraitMethodParameter {
    fn from(pat_type: PatType) -> Self {
        let name = if let Pat::Ident(ident) = pat_type.pat.as_ref() {
            ident.ident.to_string()
        } else {
            unimplemented!()
        };
        let type_ident = if let Type::Path(path) = pat_type.ty.as_ref() {
            let t = quote! { #path };
            let t = t.to_string();
            TraitMethodParameterType::Type(t)
        } else if let Type::Reference(_) = pat_type.ty.as_ref() {
            // TODO: we should check whether this is actually a &str or not
            TraitMethodParameterType::String
        } else {
            unimplemented!()
        };
        TraitMethodParameter { name, type_ident }
    }
}

#[derive(Debug, Clone)]
pub enum TraitMethodReturnType {
    Unit,
    Option(Box<TraitMethodReturnType>),
    Vec(Box<TraitMethodReturnType>),
    Type(String),
}

impl From<ReturnType> for TraitMethodReturnType {
    // we unwrap Futures and Results as the trait we're working on has only async Result return types
    fn from(return_type: ReturnType) -> Self {
        match return_type {
            ReturnType::Default => TraitMethodReturnType::Unit,
            ReturnType::Type(_, p) => match p.as_ref() {
                Type::Path(p) => {
                    let p = unwrap_result(p);

                    TraitMethodReturnType::from(p)
                }
                _ => unimplemented!(),
            },
        }
    }
}

impl From<Type> for TraitMethodReturnType {
    fn from(path: Type) -> Self {
        match path {
            Type::Path(ref type_path) => {
                let segment = type_path.path.segments.first().unwrap();
                let ident = segment.ident.to_string();
                if ident == "Vec" {
                    let p = unwrap_result(type_path);
                    TraitMethodReturnType::Vec(Box::new(TraitMethodReturnType::from(p)))
                } else if ident == "Option" {
                    let p = unwrap_result(type_path);
                    TraitMethodReturnType::Option(Box::new(TraitMethodReturnType::from(p)))
                } else {
                    let p = quote! { #path };
                    TraitMethodReturnType::Type(p.to_string())
                }
            }
            _ => {
                let p = quote! { #path };
                TraitMethodReturnType::Type(p.to_string())
            }
        }
    }
}

fn unwrap_result(p: &TypePath) -> Type {
    if let Some(path) = p.path.segments.first() {
        match path.arguments {
            PathArguments::AngleBracketed(ref args) => args
                .args
                .iter()
                .filter_map(|arg| match arg {
                    GenericArgument::Type(arg_type) => Some(arg_type.clone()),
                    _ => None,
                })
                .collect::<Vec<Type>>()
                .first()
                .unwrap()
                .clone(),
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

impl ToTokens for TraitSignature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! { let mut methods = Vec::new(); });
        for method in &self.0 {
            tokens.extend(quote! { #method });
        }
        tokens.extend(quote! { methods });
    }
}

impl ToTokens for TraitMethodSignature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let return_type = &self.return_type;
        let params: TokenStream = self
            .parameters
            .iter()
            .map(|param| quote! { parameters.push(#param); })
            .collect();
        tokens.extend(quote! {
            {
                let mut parameters = Vec::new();
                #params
                methods.push(rustic_reflect::TraitMethodSignature {
                    name: String::from(#name),
                    parameters,
                    return_type: #return_type
                });
            }
        });
    }
}

impl ToTokens for TraitMethodParameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let ident = &self.type_ident;
        tokens.extend(quote! { rustic_reflect::TraitMethodParameter {
            name: String::from(#name),
            type_ident: #ident
        }});
    }
}

impl ToTokens for TraitMethodReturnType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = match self {
            TraitMethodReturnType::Unit => quote! { rustic_reflect::TraitMethodReturnType::Unit },
            TraitMethodReturnType::Option(return_type) => {
                quote! { rustic_reflect::TraitMethodReturnType::Option(Box::new(#return_type)) }
            }
            TraitMethodReturnType::Type(ref p) => {
                quote! { rustic_reflect::TraitMethodReturnType::Type(String::from(#p)) }
            }
            TraitMethodReturnType::Vec(return_types) => {
                quote! { rustic_reflect::TraitMethodReturnType::Vec(Box::new(#return_types)) }
            }
        };
        tokens.extend(token);
    }
}

impl ToTokens for TraitMethodParameterType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = match self {
            TraitMethodParameterType::String => {
                quote! { rustic_reflect::TraitMethodParameterType::String }
            }
            TraitMethodParameterType::Type(p_type) => {
                quote! { rustic_reflect::TraitMethodParameterType::Type(String::from(#p_type)) }
            }
        };
        tokens.extend(token)
    }
}
