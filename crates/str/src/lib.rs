#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Creates a new instance of [String](https://doc.rust-lang.org/stable/std/string/struct.String.html)
#[proc_macro]
pub fn str(input: TokenStream) -> TokenStream {
    // empty string:
    if input.is_empty() {
        return quote! { ::std::string::String::new() }.into();
    }
    
    // string format:
    let Format { expr, args } = syn::parse_macro_input!(input as Format);
    
    match expr {
        // literal string:
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(fmt), .. }) => {
            // str!("a format {}..", "text")
            if args.is_some() {
                quote! { ::std::format!(#fmt #args) }
            }
            // str!("a format {text}..")
            else if fmt.value().contains(&['{', '}'][..])  {
                quote! { ::std::format!(#fmt) }
            }
            // str!("a simple text..")
            else {
                quote! { #fmt.to_owned() }
            }
        },

        // not a literal string:
        _ => quote! { #expr.to_string() }
    }.into()
}

// The string formatter
struct Format {
    pub expr: syn::Expr,
    pub args: Option<TokenStream2>,
}

impl syn::parse::Parse for Format {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse expression:
        let expr = input.parse()?;

        // parse arguments:
        let args = if input.peek(syn::token::Comma) {
            Some(input.parse()?)
        } else {
            None
        };
        
        Ok(Self { expr, args })
    }
}
