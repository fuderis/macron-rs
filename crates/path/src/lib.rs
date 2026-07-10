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

    // prepare the raw string expression: either a formatted literal or a passed expression
    let raw_expr = if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(lit_str),
        ..
    }) = &expr
    {
        if let Some(args) = args {
            quote! { ::std::format!(#lit_str #args) }
        } else {
            quote! { ::std::format!(#lit_str) }
        }
    } else {
        quote! { (#expr).to_string() }
    };

    // unified cross-platform path resolution engine
    quote! {{
        let path_raw = #raw_expr;
        let path_str: &str = path_raw.as_ref();

        if path_str.starts_with('$') || path_str.starts_with('~') {
            let mut path_normalized = path_str.replace("\\", "/");

            // handle tilde home directory aliases right away
            if path_normalized == "~" {
                path_normalized = "$home".to_string();
            } else if path_normalized.starts_with("~/") {
                path_normalized = ::std::format!("$home/{}", &path_normalized[2..]);
            }

            if path_normalized == "$" {
                // return current executable path
                ::std::env::current_exe().expect("Failed to get executable path")
            } else if path_normalized.starts_with("$/") {
                // return parent directory of the executable
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
                // parse the prefix token (e.g., "$config$/settings.toml" -> "config$")
                let token_end = path_normalized.find('/').unwrap_or(path_normalized.len());
                let token = &path_normalized[1..token_end];
                let rest = if token_end < path_normalized.len() { &path_normalized[token_end + 1..] } else { "" };

                let (key, has_app) = if token.ends_with('$') {
                    (&token[..token.len() - 1], true)
                } else {
                    (token, false)
                };

                let mut base = match key {
                    // Linux: /home/user | /home/user/.ovsy
                    // macOS: /Users/user | /Users/user/.ovsy
                    // Windows: C:\Users\user | C:\Users\user\ovsy
                    "home" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                        #[cfg(not(target_os = "windows"))] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found");
                            if has_app { p.push(::std::format!(".{}", APP_NAME)); }
                            p
                        }
                    }

                    // Linux: ~/.config | ~/.config/ovsy
                    // macOS: ~/Library/Application Support | ~/Library/Application Support/ovsy
                    // Windows: %APPDATA% | %APPDATA%\ovsy\config
                    "config" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("APPDATA").map(::std::path::PathBuf::from).expect("APPDATA not found");
                            if has_app { p.push(APP_NAME); p.push("config"); }
                            p
                        }
                        #[cfg(target_os = "macos")] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Application Support");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let mut p = ::std::env::var("XDG_CONFIG_HOME").map(::std::path::PathBuf::from)
                                .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".config")))
                                .expect("Failed to resolve config directory");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                    }

                    // Linux: ~/.local/share | ~/.local/share/ovsy
                    // macOS: ~/Library/Application Support | ~/Library/Application Support/ovsy
                    // Windows: %LOCALAPPDATA% | %LOCALAPPDATA%\ovsy\data
                    "share" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("LOCALAPPDATA").map(::std::path::PathBuf::from).expect("LOCALAPPDATA not found");
                            if has_app { p.push(APP_NAME); p.push("data"); }
                            p
                        }
                        #[cfg(target_os = "macos")] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Application Support");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let mut p = ::std::env::var("XDG_DATA_HOME").map(::std::path::PathBuf::from)
                                .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".local/share")))
                                .expect("Failed to resolve data directory");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                    }

                    // Linux: ~/.cache | ~/.cache/ovsy
                    // macOS: ~/Library/Caches | ~/Library/Caches/ovsy
                    // Windows: %LOCALAPPDATA% | %LOCALAPPDATA%\ovsy\cache
                    "cache" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("LOCALAPPDATA").map(::std::path::PathBuf::from).expect("LOCALAPPDATA not found");
                            if has_app { p.push(APP_NAME); p.push("cache"); }
                            p
                        }
                        #[cfg(target_os = "macos")] {
                            let mut p = ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found").join("Library/Caches");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let mut p = ::std::env::var("XDG_CACHE_HOME").map(::std::path::PathBuf::from)
                                .or_else(|_| ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".cache")))
                                .expect("Failed to resolve cache directory");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                    }

                    // Linux/macOS: /tmp | /tmp/ovsy
                    // Windows: %LOCALAPPDATA%\Temp | %LOCALAPPDATA%\Temp\ovsy
                    "temp" => {
                        let mut p = ::std::env::temp_dir();
                        if has_app { p.push(APP_NAME); }
                        p
                    }

                    // Linux: /opt | /opt/ovsy
                    // macOS: /Applications | /Applications/ovsy
                    // Windows: %PROGRAMFILES% | %PROGRAMFILES%\ovsy
                    "global" => {
                        #[cfg(target_os = "windows")] {
                            let global_p = ::std::env::var("PROGRAMFILES").map(::std::path::PathBuf::from).expect("PROGRAMFILES not found");
                            let local_p = ::std::env::var("LOCALAPPDATA").map(|l| ::std::path::PathBuf::from(l).join("Programs")).expect("LOCALAPPDATA not found");
                            if has_app {
                                let mut p = global_p.join(APP_NAME);
                                if p.exists() { p } else { local_p.join(APP_NAME) }
                            } else { global_p }
                        }
                        #[cfg(target_os = "macos")] {
                            let global_p = ::std::path::PathBuf::from("/Applications");
                            let local_p = ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join("Applications")).expect("HOME not found");
                            if has_app {
                                let mut p = global_p.join(APP_NAME);
                                if p.exists() || global_p.join(::std::format!("{}.app", APP_NAME)).exists() { p } else { local_p.join(APP_NAME) }
                            } else { global_p }
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let global_p = ::std::path::PathBuf::from("/opt");
                            let local_p = ::std::env::var("HOME").map(|h| ::std::path::PathBuf::from(h).join(".local/opt")).expect("HOME not found");
                            if has_app {
                                let mut p = global_p.join(APP_NAME);
                                if p.exists() { p } else { local_p.join(APP_NAME) }
                            } else { global_p }
                        }
                    }

                    // Linux: ~/.local/opt | ~/.local/opt/ovsy
                    // macOS: ~/Applications | ~/Applications/ovsy
                    // Windows: %LOCALAPPDATA%\Programs | %LOCALAPPDATA%\Programs\ovsy
                    "local" => {
                        #[cfg(target_os = "windows")] {
                            let mut p = ::std::env::var("LOCALAPPDATA")
                                .map(|l| ::std::path::PathBuf::from(l).join("Programs"))
                                .expect("LOCALAPPDATA not found");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                        #[cfg(target_os = "macos")] {
                            let mut p = ::std::env::var("HOME")
                                .map(|h| ::std::path::PathBuf::from(h).join("Applications"))
                                .expect("HOME not found");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                        #[cfg(all(unix, not(target_os = "macos")))] {
                            let mut p = ::std::env::var("HOME")
                                .map(|h| ::std::path::PathBuf::from(h).join(".local/opt"))
                                .expect("HOME not found");
                            if has_app { p.push(APP_NAME); }
                            p
                        }
                    }

                    // Linux/macOS/Windows: ~/Downloads | ~/Downloads/ovsy
                    "downloads" => {
                        let mut p = {
                            #[cfg(target_os = "windows")] { ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found") }
                            #[cfg(not(target_os = "windows"))] { ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found") }
                        };
                        p.push("Downloads");
                        if has_app { p.push(APP_NAME); }
                        p
                    }

                    // Linux/macOS/Windows: ~/Documents | ~/Documents/ovsy
                    "documents" => {
                        let mut p = {
                            #[cfg(target_os = "windows")] { ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found") }
                            #[cfg(not(target_os = "windows"))] { ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found") }
                        };
                        p.push("Documents");
                        if has_app { p.push(APP_NAME); }
                        p
                    }

                    // Linux/macOS/Windows: ~/Music | ~/Music/ovsy
                    "music" => {
                        let mut p = {
                            #[cfg(target_os = "windows")] { ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found") }
                            #[cfg(not(target_os = "windows"))] { ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found") }
                        };
                        p.push("Music");
                        if has_app { p.push(APP_NAME); }
                        p
                    }

                    // Linux/macOS/Windows: ~/Pictures | ~/Pictures/ovsy
                    "pictures" => {
                        let mut p = {
                            #[cfg(target_os = "windows")] { ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found") }
                            #[cfg(not(target_os = "windows"))] { ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found") }
                        };
                        p.push("Pictures");
                        if has_app { p.push(APP_NAME); }
                        p
                    }

                    // Linux/Windows: ~/Videos | ~/Videos/ovsy
                    // macOS: ~/Movies | ~/Movies/ovsy
                    "videos" => {
                        let mut p = {
                            #[cfg(target_os = "windows")] { ::std::env::var("USERPROFILE").map(::std::path::PathBuf::from).expect("USERPROFILE not found") }
                            #[cfg(not(target_os = "windows"))] { ::std::env::var("HOME").map(::std::path::PathBuf::from).expect("HOME not found") }
                        };
                        #[cfg(target_os = "macos")] { p.push("Movies"); }
                        #[cfg(not(target_os = "macos"))] { p.push("Videos"); }
                        if has_app { p.push(APP_NAME); }
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
    }}
    .into()
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
