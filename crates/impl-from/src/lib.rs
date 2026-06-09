#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

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

    let global_impls = read_attr_values(&attrs, struct_fields)
        .into_iter()
        .filter_map(|attr| match attr {
            AttrValue::Custom { ty, expr } => Some(quote! {
                impl ::std::convert::From<#ty> for #ident {
                    fn from(value: #ty) -> Self {
                        #expr
                    }
                }
            }),
            AttrValue::Skip => None,
        });

    match &data {
        syn::Data::Struct(st) => {
            let mut struct_field_impls = Vec::new();

            let has_skip = read_attr_values(&attrs, struct_fields)
                .iter()
                .any(|a| matches!(a, AttrValue::Skip));

            if st.fields.len() == 1 && !has_skip {
                let field = st.fields.iter().next().unwrap();
                let ty = &field.ty;
                let body = match &field.ident {
                    Some(id) => quote! { Self { #id: value } },
                    None => quote! { Self(value) },
                };

                struct_field_impls.push(quote! {
                    impl ::std::convert::From<#ty> for #ident {
                        fn from(value: #ty) -> Self {
                            #body
                        }
                    }
                });
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
                    let parsed_attrs = read_attr_values(&attrs, Some(fields));

                    if parsed_attrs.iter().any(|a| matches!(a, AttrValue::Skip)) {
                        return quote! {};
                    }

                    let mut generated_for_variant = Vec::new();

                    if fields.len() == 1 {
                        let field = fields.iter().next().unwrap();
                        let ty = &field.ty;
                        let base_output = match fields {
                            syn::Fields::Named(_) => {
                                let field_ident = &field.ident;
                                quote! { Self::#var_ident { #field_ident: value } }
                            }
                            syn::Fields::Unnamed(_) => quote! { Self::#var_ident(value) },
                            syn::Fields::Unit => quote! { Self::#var_ident },
                        };

                        generated_for_variant.push(quote! {
                            impl ::std::convert::From<#ty> for #ident {
                                fn from(value: #ty) -> Self {
                                    #base_output
                                }
                            }
                        });
                    }

                    for attr in parsed_attrs {
                        if let AttrValue::Custom { ty, expr } = attr {
                            let custom_output = match &fields {
                                syn::Fields::Named(_) => quote! { Self::#var_ident { #expr } },
                                syn::Fields::Unnamed(_) => quote! { Self::#var_ident(#expr) },
                                syn::Fields::Unit => quote! { Self::#var_ident },
                            };
                            generated_for_variant.push(quote! {
                                impl ::std::convert::From<#ty> for #ident {
                                    fn from(value: #ty) -> Self {
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

        _ => panic!("Expected a 'struct' or 'enum'"),
    }
}

enum AttrValue {
    Custom { ty: syn::Type, expr: TokenStream2 },
    Skip,
}

fn read_attr_values(attrs: &[syn::Attribute], _fields: Option<&syn::Fields>) -> Vec<AttrValue> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("from"))
        .map(|attr| match &attr.meta {
            syn::Meta::List(list) => {
                let args: FromArgs = list.parse_args().expect("Invalid arguments");
                match args {
                    FromArgs::With(ty, path) => AttrValue::Custom {
                        ty: ty.clone(),
                        expr: quote! { #path(value) },
                    },
                    FromArgs::Expr(ty, expr) => AttrValue::Custom {
                        ty: ty.clone(),
                        expr: quote! { #expr },
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
