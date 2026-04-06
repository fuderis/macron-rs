#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

//! See the documentation here [macron documentation](https://docs.rs/macron)

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;

/// The key-value collection parser
#[proc_macro]
pub fn parse_map(input: TokenStream) -> TokenStream {
    let Map { fields } = syn::parse_macro_input!(input as Map);

    let list = fields.into_iter().map(|pair| {
        let (k, v) = (pair.key, pair.value);
        quote! { (#k, #v) }
    });

    quote! { [#(#list),*] }.into()
}

/// The key-value collection
struct Map {
    pub fields: Punctuated<Field, syn::Token![,]>,
}

impl syn::parse::Parse for Map {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields = Punctuated::<Field, syn::Token![,]>::parse_terminated(input)?;

        Ok(Map { fields })
    }
}

/// The key-value collection field
struct Field {
    pub key: syn::Expr,
    pub value: syn::Expr,
}

impl syn::parse::Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;

        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
        } else if input.peek(syn::Token![=>]) {
            input.parse::<syn::Token![=>]>()?;
        } else {
            return Err(syn::Error::new(
                input.span(),
                "the expected separator is `:` or `=>` between key and value",
            ));
        }

        let value = input.parse()?;

        Ok(Field { key, value })
    }
}
