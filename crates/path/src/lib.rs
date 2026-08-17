#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Creates a new instance of PathBuf with cross-platform system layouts
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    // empty string fallback
    if input.is_empty() {
        return quote! { ::std::path::PathBuf::new() }.into();
    }

    let Format { expr, args } = syn::parse_macro_input!(input as Format);

    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(lit_str),
        ..
    }) = &expr
    {
        return parse_literal_path(lit_str, args).into();
    }

    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
    let hidden_pkg_name = format!(".{}", pkg_name);

    quote! {{
        let path_owner = #expr;
        let path_str: &str = path_owner.as_ref();

        if path_str.starts_with('$') || path_str.starts_with('~') {
            let mut path_normalized = path_str.replace("\\", "/");

            if path_normalized == "~" {
                path_normalized = "$home".to_string();
            } else if path_normalized.starts_with("~/") {
                path_normalized = ::std::format!("$home/{}", &path_normalized[2..]);
            }

            if path_normalized == "$" {
                ::std::env::current_exe().expect("Failed to get executable path")
            } else if path_normalized.starts_with("$/") {
                let rest = &path_normalized[2..];
                let mut p = ::std::env::current_exe()
                    .expect("Failed to get executable path")
                    .parent()
                    .map(::std::path::PathBuf::from)
                    .expect("Failed to get executable directory");
                if !rest.is_empty() {
                    p.push(rest);
                }
                p
            } else {
                let token_end = path_normalized.find('/').unwrap_or(path_normalized.len());
                let token = &path_normalized[1..token_end];
                let rest = if token_end < path_normalized.len() { &path_normalized[token_end + 1..] } else { "" };

                let (key, has_app) = if token.ends_with('$') {
                    (&token[..token.len() - 1], true)
                } else {
                    (token, false)
                };

                let mut base = match key {
                    "home" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                        #[cfg(not(target_os = "windows"))] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found");
                            if has_app { p.push(#hidden_pkg_name); }
                            p
                        }
                    }
                    "ssh" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found");
                            p.push(".ssh");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                        #[cfg(target_os = "macos")] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found");
                            p.push(".ssh");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found");
                            p.push(".ssh");
                            if has_app { p.push(#hidden_pkg_name); }
                            p
                        }
                    }
                    "config" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("APPDATA").map(::std::path::PathBuf::from).expect("APPDATA not found");
                            if has_app { p.push(#pkg_name); p.push("config"); }
                            p
                        }
                        #[cfg(target_os = "macos")] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Application Support");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let mut p = ::std::env::var("XDG_CONFIG_HOME").map(::std::path::PathBuf::from)
                                .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".config")))
                                .expect("Failed to resolve config directory");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                    }
                    "share" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("LOCALAPPDATA").map(::std::path::PathBuf::from).expect("LOCALAPPDATA not found");
                            if has_app { p.push(#pkg_name); p.push("data"); }
                            p
                        }
                        #[cfg(target_os = "macos")] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Application Support");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let mut p = ::std::env::var("XDG_DATA_HOME").map(::std::path::PathBuf::from)
                                .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".local/share")))
                                .expect("Failed to resolve data directory");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                    }
                    "state" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("LOCALAPPDATA").map(::std::path::PathBuf::from).expect("LOCALAPPDATA not found");
                            if has_app { p.push(#pkg_name); p.push("state"); }
                            p
                        }
                        #[cfg(target_os = "macos")] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Application Support");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let mut p = ::std::env::var("XDG_STATE_HOME").map(::std::path::PathBuf::from)
                                .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".local/state")))
                                .expect("Failed to resolve state directory");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                    }
                    "cache" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("LOCALAPPDATA").map(::std::path::PathBuf::from).expect("LOCALAPPDATA not found");
                            if has_app { p.push(#pkg_name); p.push("cache"); }
                            p
                        }
                        #[cfg(target_os = "macos")] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Caches");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let mut p = ::std::env::var("XDG_CACHE_HOME").map(::std::path::PathBuf::from)
                                .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".cache")))
                                .expect("Failed to resolve cache directory");
                            if has_app { p.push(#pkg_name); }
                            p
                        }
                    }
                    "temp" => {
                        let mut p = ::std::env::temp_dir();
                        if has_app { p.push(#pkg_name); }
                        p
                    }
                    "downloads" | "documents" | "music" | "pictures" => {
                        let dir_name = match key {
                            "downloads" => "Downloads",
                            "documents" => "Documents",
                            "music" => "Music",
                            "pictures" => "Pictures",
                            _ => unreachable!(),
                        };
                        let mut p = {
                            #[cfg(target_os = "windows")] { ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found") }
                            #[cfg(not(target_os = "windows"))] { ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found") }
                        };
                        p.push(dir_name);
                        if has_app { p.push(#pkg_name); }
                        p
                    }
                    "videos" => {
                        let mut p = {
                            #[cfg(target_os = "windows")] { ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found") }
                            #[cfg(not(target_os = "macos"))] { ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found") }
                        };
                        #[cfg(target_os = "macos")] { p.push("Movies"); }
                        #[cfg(not(target_os = "macos"))] { p.push("Videos"); }
                        if has_app { p.push(#pkg_name); }
                        p
                    }
                    _ => panic!("Unknown path prefix: ${}", key),
                };

                if !rest.is_empty() {
                    base.push(rest);
                }
                base
            }
        } else {
            ::std::path::PathBuf::from(path_str)
        }
    }}.into()
}

fn parse_literal_path(lit_str: &syn::LitStr, args: Option<TokenStream2>) -> TokenStream2 {
    let path_raw = lit_str.value();

    let make_rest_expr = |rest_str: &str| {
        let has_brackets = rest_str.contains(&['{', '}'][..]);
        if args.is_some() {
            quote! { ::std::format!(#rest_str #args) }
        } else if has_brackets {
            quote! { ::std::format!(#rest_str) }
        } else {
            quote! { #rest_str }
        }
    };

    if !path_raw.starts_with('$') && !path_raw.starts_with('~') {
        let expr = make_rest_expr(&path_raw);
        return quote! { ::std::path::PathBuf::from(#expr) };
    }

    let mut path_normalized = path_raw.replace("\\", "/");

    if path_normalized == "~" {
        path_normalized = "$home".to_string();
    } else if path_normalized.starts_with("~/") {
        path_normalized = ::std::format!("$home/{}", &path_normalized[2..]);
    }

    if path_normalized == "$" {
        return quote! { ::std::env::current_exe().expect("Failed to get executable path") };
    } else if path_normalized.starts_with("$/") {
        let rest_expr = make_rest_expr(&path_normalized[2..]);
        return quote! {{
            let mut p = ::std::env::current_exe()
                .expect("Failed to get executable path")
                .parent()
                .map(::std::path::PathBuf::from)
                .expect("Failed to get executable directory");
            let rest = #rest_expr;
            if !rest.is_empty() {
                p.push(rest);
            }
            p
        }};
    }

    let token_end = path_normalized.find('/').unwrap_or(path_normalized.len());
    let token = &path_normalized[1..token_end];
    let rest = if token_end < path_normalized.len() {
        &path_normalized[token_end + 1..]
    } else {
        ""
    };

    let (key, has_app) = if token.ends_with('$') {
        (&token[..token.len() - 1], true)
    } else {
        (token, false)
    };

    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
    let hidden_pkg_name = format!(".{}", pkg_name);

    let app_push = if has_app {
        quote! {
            #[cfg(target_os = "windows")] { p.push(#pkg_name); }
            #[cfg(not(target_os = "windows"))] { p.push(#hidden_pkg_name); }
        }
    } else {
        quote! {}
    };

    let base_tokens = match key {
        "home" => quote! {
            #[cfg(target_os = "windows")] {
                let mut p = ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found");
                #app_push
                p
            }
            #[cfg(not(target_os = "windows"))] {
                let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found");
                #app_push
                p
            }
        },
        "ssh" => quote! {
            #[cfg(target_os = "windows")] {
                let mut p = ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found");
                p.push(".ssh");
                #app_push
                p
            }
            #[cfg(target_os = "macos")] {
                let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found");
                p.push(".ssh");
                #app_push
                p
            }
            #[cfg(all(unix, not(target_os = "macos")))] {
                let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found");
                p.push(".ssh");
                #app_push
                p
            }
        },
        "config" => quote! {
            #[cfg(target_os = "windows")] {
                let mut p = ::std::env::var("APPDATA").map(::std::path::PathBuf::from).expect("APPDATA not found");
                if #has_app { p.push(#pkg_name); p.push("config"); }
                p
            }
            #[cfg(target_os = "macos")] {
                let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Application Support");
                if #has_app { p.push(#pkg_name); }
                p
            }
            #[cfg(all(unix, not(target_os = "macos")))] {
                let mut p = ::std::env::var("XDG_CONFIG_HOME").map(::std::path::PathBuf::from)
                    .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".config")))
                    .expect("Failed to resolve config directory");
                if #has_app { p.push(#pkg_name); }
                p
            }
        },
        "share" => quote! {
            #[cfg(target_os = "windows")] {
                let mut p = ::std::env::var("LOCALAPPDATA").map(::std::path::PathBuf::from).expect("LOCALAPPDATA not found");
                if #has_app { p.push(#pkg_name); p.push("data"); }
                p
            }
            #[cfg(target_os = "macos")] {
                let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Application Support");
                if #has_app { p.push(#pkg_name); }
                p
            }
            #[cfg(all(unix, not(target_os = "macos")))] {
                let mut p = ::std::env::var("XDG_DATA_HOME").map(::std::path::PathBuf::from)
                    .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".local/share")))
                    .expect("Failed to resolve data directory");
                if #has_app { p.push(#pkg_name); }
                p
            }
        },
        "state" => quote! {
            #[cfg(target_os = "windows")] {
                let mut p = ::std::env::var("LOCALAPPDATA").map(::std::path::PathBuf::from).expect("LOCALAPPDATA not found");
                if #has_app { p.push(#pkg_name); p.push("state"); }
                p
            }
            #[cfg(target_os = "macos")] {
                let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Application Support");
                if #has_app { p.push(#pkg_name); }
                p
            }
            #[cfg(all(unix, not(target_os = "macos")))] {
                let mut p = ::std::env::var("XDG_STATE_HOME").map(::std::path::PathBuf::from)
                    .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".local/state")))
                    .expect("Failed to resolve state directory");
                if #has_app { p.push(#pkg_name); }
                p
            }
        },
        "cache" => quote! {
            #[cfg(target_os = "windows")] {
                let mut p = ::std::env::var("LOCALAPPDATA").map(::std::path::PathBuf::from).expect("LOCALAPPDATA not found");
                if #has_app { p.push(#pkg_name); p.push("cache"); }
                p
            }
            #[cfg(target_os = "macos")] {
                let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Caches");
                if #has_app { p.push(#pkg_name); }
                p
            }
            #[cfg(all(unix, not(target_os = "macos")))] {
                let mut p = ::std::env::var("XDG_CACHE_HOME").map(::std::path::PathBuf::from)
                    .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".cache")))
                    .expect("Failed to resolve cache directory");
                if #has_app { p.push(#pkg_name); }
                p
            }
        },
        "temp" => quote! {{
            let mut p = ::std::env::temp_dir();
            if #has_app { p.push(#pkg_name); }
            p
        }},
        "downloads" | "documents" | "music" | "pictures" => {
            let dir_name = match key {
                "downloads" => "Downloads",
                "documents" => "Documents",
                "music" => "Music",
                "pictures" => "Pictures",
                _ => unreachable!(),
            };
            quote! {{
                let mut p = {
                    #[cfg(target_os = "windows")] { ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found") }
                    #[cfg(not(target_os = "windows"))] { ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found") }
                };
                p.push(#dir_name);
                if #has_app { p.push(#pkg_name); }
                p
            }}
        }
        "videos" => quote! {{
            let mut p = {
                #[cfg(target_os = "windows")] { ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found") }
                #[cfg(not(target_os = "macos"))] { ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found") }
            };
            #[cfg(target_os = "macos")] { p.push("Movies"); }
            #[cfg(not(target_os = "macos"))] { p.push("Videos"); }
            if #has_app { p.push(#pkg_name); }
            p
        }},
        _ => panic!("Unknown path prefix: ${}", key),
    };

    let rest_expr = make_rest_expr(rest);
    quote! {{
        let mut base = { #base_tokens };
        let rest = #rest_expr;
        if !rest.is_empty() {
            base.push(rest);
        }
        base
    }}
}

struct Format {
    pub expr: syn::Expr,
    pub args: Option<TokenStream2>,
}

impl syn::parse::Parse for Format {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;
        let args = if input.peek(syn::token::Comma) {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self { expr, args })
    }
}
