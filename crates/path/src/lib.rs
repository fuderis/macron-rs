#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Creates a new instance of PathBuf
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    // empty string:
    if input.is_empty() {
        return quote! { ::std::path::PathBuf::new() }.into();
    }
    
    // path formatting:
    let Format { expr, args } = syn::parse_macro_input!(input as Format);
    
    if let syn::Expr::Lit( syn::ExprLit { lit: syn::Lit::Str(lit_str), .. } ) = expr {
        let path_str = lit_str.value().replace("\\", "/");
        let args = if let Some(args) = args { quote!{#args} }else{ quote!{} };
        
        // executable file path:
        if path_str == "$" {
            quote! {{
                ::std::env::current_exe()
                    .expect("Failed to get executable file path")
            }}
        }
        
        // executable file dir path:
        else if path_str.starts_with("$/") {
            let path_str = &path_str[2..];
            let lit_str = syn::LitStr::new(path_str, proc_macro2::Span::call_site());
            
            quote! {{
                let exe_dir = ::std::env::current_exe().expect("Failed to get executable file path")
                    .parent()
                    .map(::std::path::PathBuf::from)
                    .expect("Failed to get binary file dir");
                exe_dir.join(&::std::format!(#lit_str #args))
            }}
        }

        // home dir path:
        else if path_str.starts_with("~/") {
            let path_str = &path_str[2..];
            let lit_str = syn::LitStr::new(path_str, proc_macro2::Span::call_site());
        
            quote! {{
                #[cfg(target_os = "windows")]
                {
                    let home_dir = ::std::env::var("USERPROFILE")
                        .map(::std::path::PathBuf::from)
                        .expect("USERPROFILE not found");
                    home_dir.join(&::std::format!(#lit_str #args))
                }
                
                #[cfg(not(target_os = "windows"))]
                {
                    let home_dir = ::std::env::var("HOME")
                        .map(::std::path::PathBuf::from)
                        .expect("HOME not found");
                    home_dir.join(&::std::format!(#lit_str #args))
                }
            }}
        }

        // user data dir path:
        else if path_str.starts_with("%/") {
            let path_str = &path_str[2..];
            let lit_str = syn::LitStr::new(path_str, proc_macro2::Span::call_site());
        
            quote! {{
                #[cfg(target_os = "windows")]
                {
                    let home_dir = ::std::env::var("APPDATA")
                        .map(::std::path::PathBuf::from)
                        .expect("Failed to get user data dir");
                    home_dir.join(&::std::format!(#lit_str #args))
                }

                #[cfg(target_os = "macos")]
                {
                    let home_dir = ::std::env::var("HOME")
                        .map(::std::path::PathBuf::from)
                        .expect("Failed to get user data dir");
                    let lib_dir = home_dir.join("Library/Application Support");
                    lib_dir.join(&::std::format!(#lit_str #args))
                }

                #[cfg(all(unix, not(target_os = "macos")))]
                {
                    let home_dir = ::std::env::var("XDG_DATA_HOME")
                        .map(::std::path::PathBuf::from)
                        .or_else(|_| 
                            ::std::env::var("HOME")
                                .map(|p| ::std::path::PathBuf::from(p).join(".local/share"))
                        )
                        .expect("Failed to get user data dir");
                    home_dir.join(&::std::format!(#lit_str #args))
                }
            }}
        }
        
        // other:
        else {
            quote! {
                ::std::path::PathBuf::from(&::std::format!(#lit_str #args))
            }
        }
    } else {
        quote! {
            ::std::path::PathBuf::from(#expr)
        }
    }
    .into()
}

/// The string formatter
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
