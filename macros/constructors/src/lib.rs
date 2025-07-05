use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_attribute]
pub fn constructors(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let generated = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let args = fields_named.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote! { #name: #ty }
                });
                let init = fields_named.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! { #name }
                });
                quote! {
                    impl #impl_generics #name #ty_generics #where_clause {
                        pub fn new(#(#args),*) -> Self {
                            Self { #(#init),* }
                        }
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let args = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let arg = format_ident!("arg{}", i);
                    let ty = &f.ty;
                    quote! { #arg: #ty }
                });
                let init = (0..fields.unnamed.len()).map(|i| {
                    let arg = format_ident!("arg{}", i);
                    quote! { #arg }
                });
                quote! {
                    impl #impl_generics #name #ty_generics #where_clause {
                        pub fn new(#(#args),*) -> Self {
                            Self(#(#init),*)
                        }
                    }
                }
            }
            Fields::Unit => quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    pub fn new() -> Self {
                        Self
                    }
                }
            },
        },

        Data::Enum(data_enum) => {
            let constructors = data_enum.variants.iter().map(|v| {
                let variant = &v.ident;
                let fn_name = format_ident!("new_{}", to_snake_case(&variant.to_string()));
                match &v.fields {
                    Fields::Unit => quote! {
                        pub fn #fn_name() -> Self {
                            Self::#variant
                        }
                    },
                    Fields::Unnamed(fields) => {
                        let args = fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let arg = format_ident!("arg{}", i);
                            let ty = &f.ty;
                            quote! { #arg: #ty }
                        });
                        let init = (0..fields.unnamed.len()).map(|i| {
                            let arg = format_ident!("arg{}", i);
                            quote! { #arg }
                        });
                        quote! {
                            pub fn #fn_name(#(#args),*) -> Self {
                                Self::#variant(#(#init),*)
                            }
                        }
                    }
                    Fields::Named(fields) => {
                        let args = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            let ty = &f.ty;
                            quote! { #name: #ty }
                        });
                        let init = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            quote! { #name }
                        });
                        quote! {
                            pub fn #fn_name(#(#args),*) -> Self {
                                Self::#variant { #(#init),* }
                            }
                        }
                    }
                }
            });

            quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #(#constructors)*
                }
            }
        }

        _ => quote! {},
    };

    quote! {
        #input
        #generated
    }
    .into()
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}
