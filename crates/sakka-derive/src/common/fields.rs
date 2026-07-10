use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Path, Type, WherePredicate, parse_quote};

use crate::{
    common,
    model::{CollectionAttr, FieldInfo, FieldKind, IgnoreAttr, OptionalAttr},
};

pub enum FieldAccessMode {
    SelfAccess,
    Binding,
}

fn wrap_layout(
    receiver: TokenStream,
    field: &FieldInfo,
    body: TokenStream,
    is_writer: bool,
) -> TokenStream {
    let with_align = common::wrap_alignment(
        receiver.clone(),
        field.attrs.align_before.as_ref(),
        field.attrs.align_after.as_ref(),
        body,
    );

    common::wrap_padding(
        receiver,
        field.attrs.pad_before.as_ref(),
        field.attrs.pad_after.as_ref(),
        with_align,
        is_writer,
    )
}

fn collection_from_field(field: &FieldInfo) -> Option<(&CollectionAttr, Type)> {
    field.attrs.collection.as_ref().map(|collection| {
        let elem_ty = match &field.kind {
            FieldKind::Vec { elem, .. } => elem.clone(),
            _ => unreachable!("collection attribute validation ensures Vec"),
        };
        (collection, elem_ty)
    })
}

fn optional_collection_from_inner<'a>(
    field: &'a FieldInfo,
    inner_ty: &Type,
) -> Option<(&'a CollectionAttr, Type)> {
    field.attrs.collection.as_ref().map(|collection| {
        let elem_ty = common::generic_inner_type(inner_ty, "Vec")
            .expect("optional Vec validation ensures Option<Vec<_>>");
        (collection, elem_ty)
    })
}

fn maybe_store_encoded_field(
    field: &FieldInfo,
    value_ref: TokenStream,
    body: TokenStream,
    generics: &Generics,
    extra_predicates: &mut Vec<WherePredicate>,
) -> TokenStream {
    let Some(store) = &field.attrs.store else {
        return body;
    };

    let store_field = &store.field;
    let store_expr = &store.expr;

    let field_ty = field.kind.ty();
    if common::type_depends_on_generics(field_ty, generics) {
        extra_predicates.push(parse_quote!(#field_ty: ::core::clone::Clone));
    }

    quote! {
        #body
        let value = ::core::clone::Clone::clone(#value_ref);
        writer.context_mut().#store_field = #store_expr;
    }
}

fn maybe_store_decoded_field(
    field: &FieldInfo,
    local: &syn::Ident,
    body: TokenStream,
    generics: &Generics,
    extra_predicates: &mut Vec<WherePredicate>,
) -> TokenStream {
    let Some(store) = &field.attrs.store else {
        return body;
    };

    let store_field = &store.field;
    let store_expr = &store.expr;

    let field_ty = field.kind.ty();
    if common::type_depends_on_generics(field_ty, generics) {
        extra_predicates.push(parse_quote!(#field_ty: ::core::clone::Clone));
    }

    quote! {
        #body
        let value = ::core::clone::Clone::clone(&#local);
        reader.context_mut().#store_field = #store_expr;
    }
}

#[allow(clippy::too_many_arguments)]
fn encode_core(
    sakka: &TokenStream,
    context_ty: &Type,
    error_ty: &Type,
    generics: &Generics,
    value_ty: &Type,
    value_expr: TokenStream,
    codec: Option<&Path>,
    collection: Option<(&CollectionAttr, Type)>,
    extra_predicates: &mut Vec<WherePredicate>,
) -> TokenStream {
    if let Some(codec) = codec {
        if common::type_depends_on_generics(value_ty, generics) {
            extra_predicates.push(
                parse_quote!( #codec: #sakka::Codec<#value_ty, #context_ty, Error = #error_ty>),
            );
        }

        quote! {
            <#codec as #sakka::Codec<#value_ty, #context_ty>>::encode(
                #value_expr,
                writer,
            )?;
        }
    } else if let Some((collection, elem_ty)) = collection {
        if common::type_depends_on_generics(&elem_ty, generics) {
            extra_predicates
                .push(parse_quote!(#elem_ty: #sakka::Encode<#context_ty, Error = #error_ty>));
        }

        match collection {
            CollectionAttr::Count(_) => {
                quote! {
                    #sakka::WriteCollection::<#context_ty>::write_slice::<#elem_ty>(
                        writer,
                        #value_expr,
                    )?;
                }
            }
            CollectionAttr::Field(_) => {
                quote! {
                    #sakka::WriteCollection::<#context_ty>::write_slice::<#elem_ty>(
                        writer,
                        #value_expr,
                    )?;
                }
            }
            CollectionAttr::Prefix(prefix) => {
                quote! {
                    #sakka::WriteCollection::<#context_ty>::write_prefixed_slice::<#elem_ty, #prefix>(
                        writer,
                        #value_expr,
                    )?;
                }
            }
        }
    } else {
        if common::type_depends_on_generics(value_ty, generics) {
            extra_predicates
                .push(parse_quote!(#value_ty: #sakka::Encode<#context_ty, Error = #error_ty>));
        }

        quote! {
            <#value_ty as #sakka::Encode<#context_ty>>::encode(#value_expr, writer)?;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn decode_core(
    sakka: &TokenStream,
    context_ty: &Type,
    error_ty: &Type,
    generics: &Generics,
    value_ty: &Type,
    codec: Option<&Path>,
    collection: Option<(&CollectionAttr, Type)>,
    extra_predicates: &mut Vec<WherePredicate>,
) -> TokenStream {
    if let Some(codec) = codec {
        if common::type_depends_on_generics(value_ty, generics) {
            extra_predicates.push(
                parse_quote!( #codec: #sakka::Codec<#value_ty, #context_ty, Error = #error_ty>),
            );
        }

        quote! {
             <#codec as #sakka::Codec<#value_ty, #context_ty>>::decode(reader)
        }
    } else if let Some((collection, elem_ty)) = collection {
        if common::type_depends_on_generics(&elem_ty, generics) {
            extra_predicates
                .push(parse_quote!(#elem_ty: #sakka::Decode<#context_ty, Error = #error_ty>));
        }

        match collection {
            CollectionAttr::Count(len) => {
                quote! {
                    #sakka::ReadCollection::<#context_ty>::read_vec::<#elem_ty>(reader, #len)
                }
            }
            CollectionAttr::Field(len_field) => {
                quote! {
                    #sakka::ReadCollection::<#context_ty>::read_vec::<#elem_ty>(
                        reader,
                        #sakka::CollectionLength::to_usize(#len_field)?,
                    )
                }
            }
            CollectionAttr::Prefix(prefix) => {
                quote! {
                    #sakka::ReadCollection::<#context_ty>::read_prefixed_vec::<#elem_ty, #prefix>(reader)
                }
            }
        }
    } else {
        if common::type_depends_on_generics(value_ty, generics) {
            extra_predicates
                .push(parse_quote!(#value_ty: #sakka::Decode<#context_ty, Error = #error_ty>));
        }

        quote! {
            <#value_ty as #sakka::Decode<#context_ty>>::decode(reader)
        }
    }
}

pub fn encode_fields(
    sakka: &TokenStream,
    context_ty: &Type,
    error_ty: &Type,
    generics: &Generics,
    fields: &[FieldInfo],
    access_mode: FieldAccessMode,
) -> (Vec<TokenStream>, Vec<WherePredicate>) {
    let mut field_encodes = Vec::new();
    let mut extra_predicates = Vec::new();

    for field in fields {
        if field.attrs.ignore.is_some() {
            continue;
        }

        let access = match access_mode {
            FieldAccessMode::SelfAccess => field.access.self_access(),
            FieldAccessMode::Binding => {
                let local = &field.local;
                quote!(#local)
            }
        };

        let access_ref = match access_mode {
            FieldAccessMode::SelfAccess => quote!(&#access),
            FieldAccessMode::Binding => quote!(#access),
        };

        if let Some(optional) = &field.attrs.optional {
            let inner_ty = match &field.kind {
                FieldKind::Option { inner, .. } => inner,
                _ => unreachable!("optional attribute validation ensures Option"),
            };

            let optional_collection = optional_collection_from_inner(field, inner_ty);

            let inner_core = encode_core(
                sakka,
                context_ty,
                error_ty,
                generics,
                inner_ty,
                quote!(__sakka_optional_inner),
                field.attrs.codec.as_ref(),
                optional_collection,
                &mut extra_predicates,
            );

            let body = match optional {
                OptionalAttr::Bool => {
                    let wrapped = common::wrap_optional(
                        sakka,
                        error_ty,
                        quote!(writer),
                        optional.clone(),
                        inner_core,
                        true,
                    );

                    wrap_layout(
                        quote!(writer),
                        field,
                        quote! {
                            let __sakka_optional_value = #access_ref;
                            #wrapped
                        },
                        true,
                    )
                }
                OptionalAttr::Eof => {
                    let with_layout = wrap_layout(quote!(writer), field, inner_core, true);
                    let wrapped = common::wrap_optional(
                        sakka,
                        error_ty,
                        quote!(writer),
                        optional.clone(),
                        with_layout,
                        true,
                    );

                    quote! {
                        let __sakka_optional_value = #access_ref;
                        #wrapped
                    }
                }
            };

            field_encodes.push(maybe_store_encoded_field(
                field,
                quote!(__sakka_optional_value),
                body,
                generics,
                &mut extra_predicates,
            ));
            continue;
        }

        let collection = collection_from_field(field);

        let core = encode_core(
            sakka,
            context_ty,
            error_ty,
            generics,
            field.kind.ty(),
            access_ref.clone(),
            field.attrs.codec.as_ref(),
            collection,
            &mut extra_predicates,
        );

        let body = wrap_layout(quote!(writer), field, core, true);
        field_encodes.push(maybe_store_encoded_field(
            field,
            access_ref,
            body,
            generics,
            &mut extra_predicates,
        ));
    }

    (field_encodes, extra_predicates)
}

pub fn decode_fields(
    sakka: &TokenStream,
    context_ty: &Type,
    error_ty: &Type,
    generics: &Generics,
    fields: &[FieldInfo],
) -> (Vec<TokenStream>, Vec<WherePredicate>) {
    let mut field_decodes = Vec::new();
    let mut extra_predicates = Vec::new();

    for field in fields {
        let name = &field.local;
        let ty = field.kind.ty();

        if let Some(optional) = &field.attrs.optional {
            let inner_ty = match &field.kind {
                FieldKind::Option { inner, .. } => inner,
                _ => unreachable!("optional attribute validation ensures Option"),
            };

            let optional_collection = optional_collection_from_inner(field, inner_ty);

            let inner_core = decode_core(
                sakka,
                context_ty,
                error_ty,
                generics,
                inner_ty,
                field.attrs.codec.as_ref(),
                optional_collection,
                &mut extra_predicates,
            );

            let body = match optional {
                OptionalAttr::Bool => {
                    let with_optional = common::wrap_optional(
                        sakka,
                        error_ty,
                        quote!(reader),
                        optional.clone(),
                        inner_core,
                        false,
                    );

                    wrap_layout(
                        quote!(reader),
                        field,
                        quote! {
                            let #name = #with_optional;
                        },
                        false,
                    )
                }
                OptionalAttr::Eof => {
                    let with_layout = wrap_layout(
                        quote!(reader),
                        field,
                        quote! {
                            let __sakka_optional_inner = (#inner_core)?;
                        },
                        false,
                    );

                    common::wrap_optional(
                        sakka,
                        error_ty,
                        quote!(reader),
                        optional.clone(),
                        quote! {
                            {
                                #with_layout
                                Some(__sakka_optional_inner)
                            }
                        },
                        false,
                    )
                }
            };

            if matches!(optional, OptionalAttr::Eof) {
                let body = quote! {
                    let #name = #body;
                };
                field_decodes.push(maybe_store_decoded_field(
                    field,
                    name,
                    body,
                    generics,
                    &mut extra_predicates,
                ));
            } else {
                field_decodes.push(maybe_store_decoded_field(
                    field,
                    name,
                    body,
                    generics,
                    &mut extra_predicates,
                ));
            }

            continue;
        }

        let body = if let Some(ignore) = &field.attrs.ignore {
            match ignore {
                IgnoreAttr::Default => {
                    if common::type_depends_on_generics(ty, generics) {
                        extra_predicates.push(parse_quote!(#ty: ::core::default::Default));
                    }

                    quote! {
                        let #name: #ty = Default::default();
                    }
                }
                IgnoreAttr::Value(value) => {
                    quote! {
                        let #name: #ty = #value;
                    }
                }
            }
        } else {
            let collection = collection_from_field(field);

            let core = decode_core(
                sakka,
                context_ty,
                error_ty,
                generics,
                ty,
                field.attrs.codec.as_ref(),
                collection,
                &mut extra_predicates,
            );

            quote! {
                let #name = (#core)?;
            }
        };

        let body = wrap_layout(quote!(reader), field, body, false);
        field_decodes.push(maybe_store_decoded_field(
            field,
            name,
            body,
            generics,
            &mut extra_predicates,
        ));
    }

    (field_decodes, extra_predicates)
}
