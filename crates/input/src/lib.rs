#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Reads user inputs from the console
#[proc_macro]
pub fn input(input: TokenStream) -> TokenStream {
    let Message { msg } = syn::parse_macro_input!(input as Message);

    // parsing message:
    let print_msg = msg
        .map(|Format { expr, args }| {
            let msg = match expr {
                // literal string:
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(fmt),
                    ..
                }) => {
                    // inputln!("a format {}..", "text")
                    if args.is_some() {
                        quote! { &::std::format!(#fmt, #args) }
                    }
                    // inputln!("a format {text}..")
                    else if fmt.value().contains(&['{', '}'][..]) {
                        quote! { &::std::format!(#fmt) }
                    }
                    // inputln!("a simple text..")
                    else {
                        quote! { #fmt }
                    }
                }

                // NOTE: chill out, this panic will never work.
                _ => panic!(),
            };

            quote! {
                use ::std::io::Write;

                ::std::io::stdout().write_all(#msg.as_bytes()).unwrap();
                ::std::io::stdout().flush().unwrap();
            }
        })
        .unwrap_or(quote! {});

    // reading user input:
    quote! {{
        #print_msg

        use ::std::io::BufRead;

        ::std::io::stdin().lock().lines()
    }}
    .into()
}

// The input message
struct Message {
    pub msg: Option<Format>,
}

impl syn::parse::Parse for Message {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            msg: input.parse().ok(),
        })
    }
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

        // check expr to literal string:
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(_),
            ..
        }) = &expr
        {
        } else {
            return Err(syn::Error::new_spanned(expr, "Expected literal string"));
        }

        // parse arguments:
        let args = if input.peek(syn::token::Comma) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { expr, args })
    }
}
