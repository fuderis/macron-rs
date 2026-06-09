use darling::{FromDeriveInput, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[derive(FromDeriveInput)]
#[darling(attributes(display), forward_attrs(allow, doc))]
struct ContainerReceiver {
    ident: syn::Ident,
    #[darling(default)]
    rename: Option<String>,
    #[darling(default)]
    fmt: Option<String>,
}

#[derive(FromVariant)]
#[darling(attributes(display))]
struct VariantReceiver {
    ident: syn::Ident,
    #[allow(dead_code)]
    fields: darling::ast::Fields<darling::util::Ignored>,
    #[darling(default)]
    rename: Option<String>,
    #[darling(default)]
    fmt: Option<String>,
}

use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToUpperCamelCase,
};

fn apply_rename(text: &str, style: Option<&str>) -> String {
    let Some(style) = style else {
        return text.to_string();
    };

    match style {
        "lowercase" => text.to_lowercase(),
        "uppercase" | "UPPERCASE" => text.to_uppercase(),
        "camelcase" | "camelCase" => text.to_lower_camel_case(),
        "CamelCase" => text.to_upper_camel_case(),
        "pascalcase" | "PascalCase" => text.to_pascal_case(),
        "snakecase" | "snake_case" => text.to_snake_case(),
        "SNAKE_CASE" | "SCREAMING_SNAKE_CASE" => text.to_shouty_snake_case(),
        "kebabcase" | "kebab-case" => text.to_kebab_case(),
        "KEBAB-CASE" | "SCREAMING-KEBAB-CASE" => text.to_shouty_kebab_case(),
        _ => panic!("Unexpected text style '{style}'. Available variants: lowercase, uppercase, UPPERCASE, camelcase, CamelCase, pascalcase, PascalCase, snakecase, snake_case, SNAKE_CASE, SCREAMING_SNAKE_CASE, kebabcase, kebab-case, KEBAB-CASE, SCREAMING-KEBAB-CASE"),
    }
}

fn read_legacy_attr(attrs: &[syn::Attribute]) -> Option<String> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("display"))
        .and_then(|attr| {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &meta.value
                {
                    return Some(s.value());
                }
            }
            None
        })
}

#[proc_macro_derive(Display, attributes(display))]
pub fn impl_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let container = match ContainerReceiver::from_derive_input(&input) {
        Ok(val) => val,
        Err(err) => return err.write_errors().into(),
    };

    let ident = &container.ident;
    let legacy_fmt = read_legacy_attr(&input.attrs);
    let container_fmt = container.fmt.or(legacy_fmt);

    let body = match &input.data {
        Data::Struct(st) => {
            if let Some(fmt) = container_fmt {
                let field_names = st
                    .fields
                    .iter()
                    .filter_map(|f| f.ident.as_ref())
                    .collect::<Vec<_>>();

                quote! {
                    #[allow(unused_variables)]
                    {
                        #( let #field_names = &self.#field_names; )*
                        write!(f, #fmt)
                    }
                }
            } else {
                let name = apply_rename(&ident.to_string(), container.rename.as_deref());
                quote! { write!(f, #name) }
            }
        }

        Data::Enum(en) => {
            let mut matches = Vec::new();

            for variant in &en.variants {
                let var_ctx = match VariantReceiver::from_variant(variant) {
                    Ok(val) => val,
                    Err(err) => return err.write_errors().into(),
                };

                let var_ident = &var_ctx.ident;
                let legacy_var_fmt = read_legacy_attr(&variant.attrs);
                let var_fmt = var_ctx.fmt.or(legacy_var_fmt);

                let match_arm = match &variant.fields {
                    Fields::Unit => {
                        if let Some(fmt) = var_fmt {
                            quote! { Self::#var_ident => write!(f, #fmt) }
                        } else {
                            let name = apply_rename(
                                &var_ident.to_string(),
                                var_ctx.rename.as_deref().or(container.rename.as_deref()),
                            );
                            quote! { Self::#var_ident => write!(f, #name) }
                        }
                    }

                    Fields::Named(fields) => {
                        let args = fields
                            .named
                            .iter()
                            .filter_map(|f| f.ident.as_ref())
                            .collect::<Vec<_>>();
                        if let Some(fmt) = var_fmt {
                            quote! {
                                Self::#var_ident { #(#args,)* .. } => {
                                    #[allow(unused_variables)]
                                    {
                                        write!(f, #fmt)
                                    }
                                }
                            }
                        } else {
                            let name = apply_rename(
                                &var_ident.to_string(),
                                var_ctx.rename.as_deref().or(container.rename.as_deref()),
                            );
                            quote! { Self::#var_ident { .. } => write!(f, #name) }
                        }
                    }

                    Fields::Unnamed(fields) => {
                        if let Some(fmt) = var_fmt {
                            let args = (0..fields.unnamed.len())
                                .map(|i| quote::format_ident!("_{}", i))
                                .collect::<Vec<_>>();

                            let used_args = (0..fields.unnamed.len())
                                .filter(|i| {
                                    fmt.contains(&format!("{{{i}}}"))
                                        || fmt.contains(&format!("{{{i}:"))
                                })
                                .map(|i| quote::format_ident!("_{}", i))
                                .collect::<Vec<_>>();

                            quote! {
                                Self::#var_ident(#(#args,)*) => {
                                    #[allow(unused_variables)]
                                    {
                                        write!(f, #fmt, #(#used_args),*)
                                    }
                                }
                            }
                        } else if fields.unnamed.len() == 1 {
                            quote! { Self::#var_ident(ref arg) => write!(f, "{}", arg) }
                        } else {
                            let name = apply_rename(
                                &var_ident.to_string(),
                                var_ctx.rename.as_deref().or(container.rename.as_deref()),
                            );
                            quote! { Self::#var_ident(..) => write!(f, #name) }
                        }
                    }
                };
                matches.push(match_arm);
            }

            if matches.is_empty() {
                quote! { write!(f, "") }
            } else {
                quote! {
                    match self {
                        #(#matches,)*
                    }
                }
            }
        }
        _ => panic!("Only structs and enums are supported"),
    };

    quote! {
        impl ::std::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                #body
            }
        }
    }
    .into()
}
