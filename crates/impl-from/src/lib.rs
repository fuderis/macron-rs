#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

/// The implementation of [std::convert::From] trait
#[proc_macro_derive(From, attributes(from))]
pub fn impl_from(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_fields = match &data {
        syn::Data::Struct(st) => Some(&st.fields),
        _ => None,
    };

    let global_attrs = read_attr_values(&attrs);
    let has_skip = global_attrs.iter().any(|a| matches!(a, AttrValue::Skip));

    let global_impls = global_attrs.into_iter().filter_map(|attr| match attr {
        AttrValue::Custom {
            source_ty,
            unbox_source,
            kind,
        } => {
            let target_field = struct_fields.and_then(|f| single_field(f));
            let expr = build_custom_expr(kind, unbox_source);
            let wrapped_expr = match target_field {
                Some(field) if is_boxed_type(&field.ty) && !is_boxed_type(&source_ty) => {
                    quote! { ::std::boxed::Box::new(#expr) }
                }
                _ => expr,
            };

            let body = match target_field {
                Some(field) => match &field.ident {
                    Some(id) => quote! { Self { #id: #wrapped_expr } },
                    None => quote! { Self(#wrapped_expr) },
                },
                None => wrapped_expr,
            };

            Some(quote! {
                impl ::std::convert::From<#source_ty> for #ident {
                    fn from(value: #source_ty) -> Self {
                        #body
                    }
                }
            })
        }
        AttrValue::Skip => None,
    });

    match &data {
        syn::Data::Struct(st) => {
            let mut struct_field_impls = Vec::new();

            if !has_skip {
                if let Some(field) = single_field(&st.fields) {
                    struct_field_impls.extend(generate_field_from_impls(
                        &ident,
                        None,
                        &syn::Fields::Unnamed(syn::FieldsUnnamed {
                            paren_token: Default::default(),
                            unnamed: std::iter::once(field.clone()).collect(),
                        }),
                        field,
                        &[],
                        true,
                    ));
                }
            }

            quote! {
                #(#global_impls)*
                #(#struct_field_impls)*
            }
            .into()
        }

        syn::Data::Enum(en) => {
            let var_impls = en.variants.iter().map(
                |syn::Variant {
                     ident: var_ident,
                     attrs,
                     fields,
                     ..
                 }| {
                    let parsed_attrs = read_attr_values(&attrs);

                    if parsed_attrs.iter().any(|a| matches!(a, AttrValue::Skip)) {
                        return quote! {};
                    }

                    let mut generated_for_variant = Vec::new();

                    if let Some(field) = single_field(fields) {
                        generated_for_variant.extend(generate_field_from_impls(
                            &ident,
                            Some(var_ident),
                            fields,
                            field,
                            &parsed_attrs,
                            false,
                        ));
                    }

                    for attr in parsed_attrs {
                        if let AttrValue::Custom {
                            source_ty,
                            unbox_source,
                            kind,
                        } = attr
                        {
                            let mut expr = build_custom_expr(kind, unbox_source);
                            if let Some(field) = single_field(fields) {
                                if is_boxed_type(&field.ty) && !is_boxed_type(&source_ty) {
                                    expr = quote! { ::std::boxed::Box::new(#expr) };
                                }
                            }

                            let custom_output = match fields {
                                syn::Fields::Named(_) => quote! { Self::#var_ident { #expr } },
                                syn::Fields::Unnamed(_) => quote! { Self::#var_ident(#expr) },
                                syn::Fields::Unit => quote! { Self::#var_ident },
                            };

                            generated_for_variant.push(quote! {
                                impl ::std::convert::From<#source_ty> for #ident {
                                    fn from(value: #source_ty) -> Self {
                                        #custom_output
                                    }
                                }
                            });
                        }
                    }

                    quote! { #(#generated_for_variant)* }
                },
            );

            quote! {
                #(#global_impls)*
                #(#var_impls)*
            }
            .into()
        }

        _ => panic!("Expected a `struct` or `enum`."),
    }
}

fn generate_field_from_impls(
    container_ident: &syn::Ident,
    variant_ident: Option<&syn::Ident>,
    fields: &syn::Fields,
    field: &syn::Field,
    parsed_attrs: &[AttrValue],
    is_struct: bool,
) -> Vec<TokenStream2> {
    let mut impls = Vec::new();
    let ty = &field.ty;

    let make_target = |inner_val: TokenStream2| -> TokenStream2 {
        if is_struct {
            match &field.ident {
                Some(id) => quote! { Self { #id: #inner_val } },
                None => quote! { Self(#inner_val) },
            }
        } else {
            let var_ident = variant_ident.expect("variant ident required for enum");
            match fields {
                syn::Fields::Named(_) => {
                    let field_ident = &field.ident;
                    quote! { Self::#var_ident { #field_ident: #inner_val } }
                }
                syn::Fields::Unnamed(_) => quote! { Self::#var_ident(#inner_val) },
                syn::Fields::Unit => quote! { Self::#var_ident },
            }
        }
    };

    // direct conversion from the field type
    let direct_body = make_target(quote! { value });
    impls.push(quote! {
        impl ::std::convert::From<#ty> for #container_ident {
            fn from(value: #ty) -> Self {
                #direct_body
            }
        }
    });

    // if field is of type Box<Inner>, allow conversion with automatic wrapping to Box
    if let Some(inner_ty) = extract_boxed_inner(ty) {
        let already_has_inner_attr = parsed_attrs.iter().any(|a| {
            matches!(
                a,
                AttrValue::Custom { source_ty, .. } if same_type(source_ty, inner_ty)
            )
        });

        if !already_has_inner_attr {
            let boxed_body = make_target(quote! { ::std::boxed::Box::new(value) });
            impls.push(quote! {
                impl ::std::convert::From<#inner_ty> for #container_ident {
                    fn from(value: #inner_ty) -> Self {
                        #boxed_body
                    }
                }
            });
        }
    }

    // if field is of type Inner, but the input is Box<Inner>, unpack the Box
    let boxed_source_ty: syn::Type = syn::parse_quote!(::std::boxed::Box<#ty>);
    let already_has_boxed_attr = parsed_attrs.iter().any(|a| {
        matches!(
            a,
            AttrValue::Custom { source_ty, .. } if is_same_box_type(source_ty, ty)
        )
    });

    if !is_boxed_type(ty) && !already_has_boxed_attr {
        let unboxed_body = make_target(quote! { *value });
        impls.push(quote! {
            impl ::std::convert::From<#boxed_source_ty> for #container_ident {
                fn from(value: #boxed_source_ty) -> Self {
                    #unboxed_body
                }
            }
        });
    }

    impls
}

fn build_custom_expr(kind: CustomKind, unbox_source: bool) -> TokenStream2 {
    let val_expr = if unbox_source {
        quote! { *value }
    } else {
        quote! { value }
    };

    match kind {
        CustomKind::With(path) => quote! { #path(#val_expr) },
        CustomKind::Expr(expr) => {
            if unbox_source {
                quote! {
                    {
                        let value = *value;
                        #expr
                    }
                }
            } else {
                quote! { #expr }
            }
        }
    }
}

fn single_field(fields: &syn::Fields) -> Option<&syn::Field> {
    if fields.len() == 1 {
        fields.iter().next()
    } else {
        None
    }
}

fn is_boxed_type(ty: &syn::Type) -> bool {
    extract_boxed_inner(ty).is_some()
}

fn extract_boxed_inner(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(type_path) = ty else {
        return None;
    };
    let last = type_path.path.segments.last()?;
    if last.ident != "Box" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &last.arguments else {
        return None;
    };
    if args.args.len() != 1 {
        return None;
    }
    match args.args.first()? {
        syn::GenericArgument::Type(inner) => Some(inner),
        _ => None,
    }
}

fn same_type(a: &syn::Type, b: &syn::Type) -> bool {
    a.to_token_stream().to_string() == b.to_token_stream().to_string()
}

fn is_same_box_type(candidate: &syn::Type, expected_inner: &syn::Type) -> bool {
    extract_boxed_inner(candidate)
        .map(|inner| same_type(inner, expected_inner))
        .unwrap_or(false)
}

enum CustomKind {
    With(syn::Path),
    Expr(syn::Expr),
}

enum AttrValue {
    Custom {
        source_ty: syn::Type,
        unbox_source: bool,
        kind: CustomKind,
    },
    Skip,
}

fn read_attr_values(attrs: &[syn::Attribute]) -> Vec<AttrValue> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("from"))
        .map(|attr| match &attr.meta {
            syn::Meta::List(list) => {
                let args: FromArgs = list.parse_args().expect("Invalid arguments");
                match args {
                    FromArgs::With(ty, path) => AttrValue::Custom {
                        unbox_source: is_boxed_type(&ty),
                        source_ty: ty,
                        kind: CustomKind::With(path),
                    },
                    FromArgs::Expr(ty, expr) => AttrValue::Custom {
                        unbox_source: is_boxed_type(&ty),
                        source_ty: ty,
                        kind: CustomKind::Expr(expr),
                    },
                    FromArgs::Skip => AttrValue::Skip,
                }
            }
            _ => panic!("Unsupported attribute format. Use #[from(skip)] or #[from(Type, ...)]"),
        })
        .collect()
}

enum FromArgs {
    With(syn::Type, syn::Path),
    Expr(syn::Type, syn::Expr),
    Skip,
}

impl syn::parse::Parse for FromArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let fork = input.fork();
            let ident: syn::Ident = fork.parse()?;
            if ident == "skip" {
                input.parse::<syn::Ident>()?;
                return Ok(FromArgs::Skip);
            }
        }

        let ty: syn::Type = input.parse()?;
        input.parse::<syn::token::Comma>()?;
        let ident: syn::Ident = input.parse()?;
        input.parse::<syn::token::Eq>()?;

        if ident == "with" {
            let path: syn::Path = input.parse()?;
            Ok(FromArgs::With(ty, path))
        } else if ident == "expr" {
            let expr: syn::Expr = input.parse()?;
            Ok(FromArgs::Expr(ty, expr))
        } else {
            Err(syn::Error::new(
                ident.span(),
                "expected `with`, `expr` or `skip`",
            ))
        }
    }
}
