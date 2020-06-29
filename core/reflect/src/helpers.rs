use syn::{GenericArgument, PathArguments, Type, TypePath};

pub fn unwrap_generic(p: &TypePath) -> Type {
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
