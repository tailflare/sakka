use alloc::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    GenericArgument, Generics, Ident, PathArguments, ReturnType, Type, TypePath, WherePredicate,
    parse_quote,
};

pub struct ImplGenerics {
    pub impl_generics: TokenStream,
    pub ty_generics: TokenStream,
    pub where_clause: TokenStream,
}

pub fn build_impl_generics(
    generics: &Generics,
    extra_predicates: impl IntoIterator<Item = WherePredicate>,
    include_ctx: bool,
) -> ImplGenerics {
    let mut impl_generics = generics.clone();
    if include_ctx {
        impl_generics.params.insert(0, parse_quote!(Ctx));
    }
    impl_generics.make_where_clause().predicates.extend(extra_predicates);

    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();
    let (_, ty_generics, _) = generics.split_for_impl();

    ImplGenerics {
        impl_generics: quote!(#impl_generics),
        ty_generics: quote!(#ty_generics),
        where_clause: quote!(#where_clause),
    }
}

pub fn type_depends_on_generics(ty: &Type, generics: &Generics) -> bool {
    let type_param_names: BTreeSet<_> =
        generics.type_params().map(|param| param.ident.clone()).collect();

    if type_param_names.is_empty() {
        return false;
    }

    type_mentions_any_type_param(ty, &type_param_names)
}

fn type_mentions_any_type_param(ty: &Type, type_param_names: &BTreeSet<Ident>) -> bool {
    match ty {
        Type::Array(array) => type_mentions_any_type_param(&array.elem, type_param_names),
        Type::Group(group) => type_mentions_any_type_param(&group.elem, type_param_names),
        Type::Paren(paren) => type_mentions_any_type_param(&paren.elem, type_param_names),
        Type::Path(type_path) => type_path_mentions_any_type_param(type_path, type_param_names),
        Type::Ptr(ptr) => type_mentions_any_type_param(&ptr.elem, type_param_names),
        Type::Reference(reference) => {
            type_mentions_any_type_param(&reference.elem, type_param_names)
        }
        Type::Slice(slice) => type_mentions_any_type_param(&slice.elem, type_param_names),
        Type::Tuple(tuple) => {
            tuple.elems.iter().any(|elem| type_mentions_any_type_param(elem, type_param_names))
        }
        _ => false,
    }
}

fn type_path_mentions_any_type_param(
    type_path: &TypePath,
    type_param_names: &BTreeSet<Ident>,
) -> bool {
    if let Some(qself) = &type_path.qself
        && type_mentions_any_type_param(&qself.ty, type_param_names)
    {
        return true;
    }

    for segment in &type_path.path.segments {
        if type_param_names.contains(&segment.ident) {
            return true;
        }

        match &segment.arguments {
            PathArguments::None => {}

            PathArguments::AngleBracketed(args) => {
                for arg in &args.args {
                    if let GenericArgument::Type(ty) = arg {
                        if type_mentions_any_type_param(ty, type_param_names) {
                            return true;
                        }
                        continue;
                    }

                    if let GenericArgument::AssocType(assoc) = arg
                        && type_mentions_any_type_param(&assoc.ty, type_param_names)
                    {
                        return true;
                    }
                }
            }

            PathArguments::Parenthesized(args) => {
                if args.inputs.iter().any(|ty| type_mentions_any_type_param(ty, type_param_names)) {
                    return true;
                }

                if let ReturnType::Type(_, ty) = &args.output
                    && type_mentions_any_type_param(ty, type_param_names)
                {
                    return true;
                }
            }
        }
    }

    false
}
