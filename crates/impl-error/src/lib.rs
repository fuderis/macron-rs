#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use quote::quote;

/// The implementation of trait [Error](std::error::Error)
#[proc_macro_derive(Error, attributes(source))]
pub fn impl_error(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, .. } = syn::parse_macro_input!(input as syn::DeriveInput);
    
    match data {
        syn::Data::Struct(st) => {
            let source = match st.fields {
                syn::Fields::Named(fields) => {
                    let src_field = fields.named
                        .iter()
                        .find(|f| f.ident
                            .as_ref()
                            .map(|i| i == "source")
                            .unwrap_or(false)
                        );
            
                    match src_field {
                        Some(field) if is_option_type(&field.ty) => quote! { self.source.as_deref().map(|e| e as &(dyn ::std::error::Error + 'static)) },
                        Some(_) => quote! { Some(&*self.source) },
                        None => quote! { None }
                    }
                },

                _ => quote! { None }
            };
            
            quote! {
                impl ::std::error::Error for #ident {
                    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
                        #source
                    }
                }
            }.into()
        },

        syn::Data::Enum(en) => {
            let vars = en.variants
                .into_iter()
                .map(|syn::Variant { ident: var_ident, fields, .. }| {
                    match fields {
                        syn::Fields::Named(fields) => {
                            let src_field = fields.named
                                .iter()
                                .find(|f| f.ident
                                    .as_ref()
                                    .map(|i| i == "source")
                                    .unwrap_or(false)
                                );
                    
                            match src_field {
                                Some(field) if is_option_type(&field.ty) => quote! { Self::#var_ident { source, .. } => source.as_deref().map(|e| e as &(dyn ::std::error::Error + 'static)) },
                                Some(_) => quote! { Self::#var_ident { source, .. } => Some(&**source) }, // double deref для ссылки на Box в паттерн-матчинге
                                None => quote! { Self::#var_ident { .. } => None }
                            }
                        },

                        syn::Fields::Unnamed(fields) => {
                             let src_field = fields.unnamed
                                .iter()
                                .enumerate()
                                .find(|(_, field)| field.attrs
                                    .iter()
                                    .any(|attr| attr.path().is_ident("source"))
                                );

                            let src_idx = src_field.map(|(idx, _)| idx).unwrap_or(0);
                            let stubs = (0..src_idx).into_iter().map(|_| quote! { _ });

                            match src_field {
                                Some((_, field)) if is_option_type(&field.ty) => quote! { Self::#var_ident(#(#stubs,)* source, ..) => source.as_deref().map(|e| e as &(dyn ::std::error::Error + 'static)) },
                                Some(_) => quote! { Self::#var_ident(#(#stubs,)* source, ..) => Some(&**source) }, // double deref здесь, т.к. source в match — это &Box или &Type
                                None => quote! { Self::#var_ident(..) => None }
                            }
                        },

                        syn::Fields::Unit => quote! { Self::#var_ident => None }
                    }
                });
            
            quote! {
                impl ::std::error::Error for #ident {
                    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
                        match self {
                            #(
                                #vars,
                            )*
                        }
                    }
                }
            }.into()
        },

        _ => panic!("the expected a 'struct' or 'enum'")
    }
}

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        type_path.path.segments.last().map_or(false, |seg| seg.ident == "Option")
    } else {
        false
    }
}
