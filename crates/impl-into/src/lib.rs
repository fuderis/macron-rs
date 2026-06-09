#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashMap;

/// The implementation of [std::convert::Into] trait
#[proc_macro_derive(Into, attributes(into))]
pub fn impl_into(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_fields = match &data {
        syn::Data::Struct(st) => Some(&st.fields),
        _ => None,
    };

    let global_attrs = read_attr_values(&attrs, struct_fields);
    let has_skip = global_attrs.iter().any(|a| matches!(a, AttrValue::Skip));

    let global_impls = global_attrs.into_iter().filter_map(|attr| match attr {
        AttrValue::Custom { ty, expr } => Some(quote! {
            impl ::std::convert::Into<#ty> for #ident {
                fn into(self) -> #ty {
                    let value = self;
                    #expr
                }
            }
        }),
        AttrValue::Skip => None,
    });

    match &data {
        syn::Data::Struct(st) => {
            let mut struct_field_impls = Vec::new();

            if st.fields.len() == 1 && !has_skip {
                let field = st.fields.iter().next().unwrap();
                let ty = &field.ty;
                let body = match &field.ident {
                    Some(id) => quote! { self.#id },
                    None => quote! { self.0 },
                };

                struct_field_impls.push(quote! {
                    impl ::std::convert::Into<#ty> for #ident {
                        fn into(self) -> #ty {
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
            // Карта: Ключ = Целевой тип, Значение = (syn::Type, Vec<MatchArm>)
            let mut target_types: HashMap<String, (syn::Type, Vec<TokenStream2>)> = HashMap::new();

            for variant in &en.variants {
                let parsed_attrs = read_attr_values(&variant.attrs, Some(&variant.fields));

                if parsed_attrs.iter().any(|a| matches!(a, AttrValue::Skip)) {
                    continue;
                }

                let var_ident = &variant.ident;

                // Собираем токены для деструктуризации текущего варианта
                let match_pattern = match &variant.fields {
                    syn::Fields::Named(fields_named) => {
                        let idents = fields_named.named.iter().map(|f| &f.ident);
                        quote! { Self::#var_ident { #(#idents),* } }
                    }
                    syn::Fields::Unnamed(fields_unnamed) => {
                        let placeholders = (0..fields_unnamed.unnamed.len())
                            .map(|i| quote::format_ident!("__{}", i));
                        quote! { Self::#var_ident ( #(#placeholders),* ) }
                    }
                    syn::Fields::Unit => quote! { Self::#var_ident },
                };

                // Создаем биндинг переменной `value`
                let binding = match &variant.fields {
                    syn::Fields::Named(fields_named) if fields_named.named.len() == 1 => {
                        let id = &fields_named.named[0].ident;
                        quote! { let value = #id; }
                    }
                    syn::Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
                        quote! { let value = __0; }
                    }
                    _ => quote! {},
                };

                // ЕСЛИ АТРИБУТОВ НЕТ: Проверяем на авто-Into (ровно 1 поле)
                if parsed_attrs.is_empty() && variant.fields.len() == 1 {
                    let field = variant.fields.iter().next().unwrap();
                    let ty = &field.ty;
                    let ty_string = quote! { #ty }.to_string();

                    let arm = quote! {
                        #match_pattern => {
                            #binding
                            value
                        }
                    };

                    target_types
                        .entry(ty_string)
                        .or_insert_with(|| (ty.clone(), Vec::new()))
                        .1
                        .push(arm);
                } else {
                    // ЕСЛИ АТРИБУТЫ ЕСТЬ: Обрабатываем каждый
                    for attr in parsed_attrs {
                        if let AttrValue::Custom { ty, expr } = attr {
                            let ty_string = quote! { #ty }.to_string();

                            let arm = quote! {
                                #match_pattern => {
                                    #binding
                                    #expr
                                }
                            };

                            target_types
                                .entry(ty_string)
                                .or_insert_with(|| (ty.clone(), Vec::new()))
                                .1
                                .push(arm);
                        }
                    }
                }
            }

            // Генерируем блоки `impl`
            let enum_impls = target_types.into_iter().map(|(_, (ty, arms))| {
                quote! {
                    impl ::std::convert::Into<#ty> for #ident {
                        fn into(self) -> #ty {
                            match self {
                                #(#arms)*
                                _ => panic!("Incompatible enum variant for Into conversion"),
                            }
                        }
                    }
                }
            });

            quote! {
                #(#global_impls)*
                #(#enum_impls)*
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
        .filter(|attr| attr.path().is_ident("into"))
        .map(|attr| match &attr.meta {
            syn::Meta::List(list) => {
                let args: IntoArgs = list.parse_args().expect("Invalid arguments");
                match args {
                    IntoArgs::With(ty, path) => AttrValue::Custom {
                        ty: ty.clone(),
                        expr: quote! { #path(value) },
                    },
                    IntoArgs::Expr(ty, expr) => AttrValue::Custom {
                        ty: ty.clone(),
                        expr: quote! { #expr },
                    },
                    IntoArgs::Skip => AttrValue::Skip,
                }
            }
            _ => panic!("Unsupported attribute format. Use #[into(skip)] or #[into(Type, ...)]"),
        })
        .collect()
}

enum IntoArgs {
    With(syn::Type, syn::Path),
    Expr(syn::Type, syn::Expr),
    Skip,
}

impl syn::parse::Parse for IntoArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let fork = input.fork();
            let ident: syn::Ident = fork.parse()?;
            if ident == "skip" {
                input.parse::<syn::Ident>()?;
                return Ok(IntoArgs::Skip);
            }
        }

        let ty: syn::Type = input.parse()?;
        input.parse::<syn::token::Comma>()?;
        let ident: syn::Ident = input.parse()?;
        input.parse::<syn::token::Eq>()?;

        if ident == "with" {
            let path: syn::Path = input.parse()?;
            Ok(IntoArgs::With(ty, path))
        } else if ident == "expr" {
            let expr: syn::Expr = input.parse()?;
            Ok(IntoArgs::Expr(ty, expr))
        } else {
            Err(syn::Error::new(
                ident.span(),
                "expected `with`, `expr` or `skip`",
            ))
        }
    }
}
