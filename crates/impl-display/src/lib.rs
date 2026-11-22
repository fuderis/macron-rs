use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToUpperCamelCase,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Attribute, Data, DeriveInput, Expr,
    Fields, Ident, Meta,
};

struct LetBinding {
    ident: Ident,
    expr: Expr,
}

struct DisplayAttr {
    fmt: Option<String>,
    rename_all: Option<String>,
    lets: Vec<LetBinding>,
}

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

fn parse_display_attrs(attrs: &[Attribute]) -> syn::Result<DisplayAttr> {
    let mut result = DisplayAttr {
        fmt: None,
        rename_all: None,
        lets: Vec::new(),
    };

    for attr in attrs {
        if !attr.path().is_ident("display") {
            continue;
        }

        match &attr.meta {
            Meta::NameValue(nv) => {
                if let Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &nv.value
                {
                    result.fmt = Some(s.value());
                } else {
                    return Err(syn::Error::new_spanned(
                        &nv.value,
                        "expected string literal in #[display = \"...\"]",
                    ));
                }
            }

            Meta::List(_) => {
                let items = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)?;
                for meta in items {
                    match meta {
                        Meta::NameValue(nv) => {
                            let Some(ident) = nv.path.get_ident() else {
                                return Err(syn::Error::new_spanned(
                                    &nv.path,
                                    "expected identifier key in #[display(...)]",
                                ));
                            };

                            if ident == "fmt" {
                                if let Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Str(s),
                                    ..
                                }) = &nv.value
                                {
                                    result.fmt = Some(s.value());
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        &nv.value,
                                        "`fmt` must be a string literal",
                                    ));
                                }
                            } else if ident == "rename_all" {
                                if let Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Str(s),
                                    ..
                                }) = &nv.value
                                {
                                    result.rename_all = Some(s.value());
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        &nv.value,
                                        "`rename_all` must be a string literal",
                                    ));
                                }
                            } else {
                                result.lets.push(LetBinding {
                                    ident: ident.clone(),
                                    expr: nv.value,
                                });
                            }
                        }
                        other => {
                            return Err(syn::Error::new_spanned(
                                other,
                                "expected `key = expr` inside #[display(...)]",
                            ));
                        }
                    }
                }
            }

            Meta::Path(path) => {
                return Err(syn::Error::new_spanned(
                    path,
                    "unexpected bare #[display] attribute",
                ));
            }
        }
    }

    Ok(result)
}

fn emit_let_bindings(bindings: &[LetBinding]) -> proc_macro2::TokenStream {
    let stmts = bindings.iter().map(|b| {
        let ident = &b.ident;
        let expr = &b.expr;
        quote! { let #ident = #expr; }
    });
    quote! { #(#stmts)* }
}

#[proc_macro_derive(Display, attributes(display))]
pub fn impl_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let container_attr = match parse_display_attrs(&input.attrs) {
        Ok(val) => val,
        Err(err) => return err.to_compile_error().into(),
    };

    let ident = &input.ident;
    let container_fmt = container_attr.fmt;
    let container_rename_all = container_attr.rename_all;
    let container_bindings = container_attr.lets;

    let body = match &input.data {
        Data::Struct(st) => {
            if let Some(fmt) = container_fmt {
                let field_bindings = st
                    .fields
                    .iter()
                    .enumerate()
                    .filter_map(|(_i, f)| {
                        f.ident.as_ref().map(|ident| {
                            quote! { let #ident = &self.#ident; }
                        })
                    })
                    .collect::<Vec<_>>();

                let positional_args: Vec<_> = match &st.fields {
                    Fields::Unnamed(fields) => (0..fields.unnamed.len())
                        .map(|i| {
                            let idx = syn::Index::from(i);
                            quote! { &self.#idx }
                        })
                        .collect(),
                    _ => Vec::new(),
                };

                let lets = emit_let_bindings(&container_bindings);

                quote! {
                    #[allow(unused_variables)]
                    {
                        #(#field_bindings)*
                        #lets
                        write!(f, #fmt #(, #positional_args)*)
                    }
                }
            } else if let Fields::Unnamed(fields) = &st.fields {
                if fields.unnamed.len() == 1 {
                    quote! { write!(f, "{}", &self.0) }
                } else {
                    let name = apply_rename(&ident.to_string(), container_rename_all.as_deref());
                    quote! { write!(f, #name) }
                }
            } else {
                let name = apply_rename(&ident.to_string(), container_rename_all.as_deref());
                quote! { write!(f, #name) }
            }
        }

        Data::Enum(en) => {
            let mut matches = Vec::new();

            for variant in &en.variants {
                let var_attr = match parse_display_attrs(&variant.attrs) {
                    Ok(val) => val,
                    Err(err) => return err.to_compile_error().into(),
                };

                let var_ident = &variant.ident;
                let var_fmt = var_attr.fmt;
                let var_rename_all = var_attr.rename_all.or_else(|| container_rename_all.clone());
                let var_bindings = var_attr.lets;

                let match_arm = match &variant.fields {
                    Fields::Unit => {
                        if let Some(fmt) = var_fmt {
                            let lets = emit_let_bindings(&var_bindings);
                            if var_bindings.is_empty() {
                                quote! { Self::#var_ident => write!(f, #fmt) }
                            } else {
                                quote! {
                                    Self::#var_ident => {
                                        #[allow(unused_variables)]
                                        {
                                            #lets
                                            write!(f, #fmt)
                                        }
                                    }
                                }
                            }
                        } else {
                            let name =
                                apply_rename(&var_ident.to_string(), var_rename_all.as_deref());
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
                            let lets = emit_let_bindings(&var_bindings);
                            quote! {
                                Self::#var_ident { #(#args,)* .. } => {
                                    #[allow(unused_variables)]
                                    {
                                        #lets
                                        write!(f, #fmt)
                                    }
                                }
                            }
                        } else {
                            let name =
                                apply_rename(&var_ident.to_string(), var_rename_all.as_deref());
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

                            let lets = emit_let_bindings(&var_bindings);
                            quote! {
                                Self::#var_ident(#(#args,)*) => {
                                    #[allow(unused_variables)]
                                    {
                                        #lets
                                        write!(f, #fmt, #(#used_args),*)
                                    }
                                }
                            }
                        } else if fields.unnamed.len() == 1 {
                            quote! { Self::#var_ident(ref arg) => write!(f, "{}", arg) }
                        } else {
                            let name =
                                apply_rename(&var_ident.to_string(), var_rename_all.as_deref());
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
        _ => panic!("Only structs and enums are supported."),
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
