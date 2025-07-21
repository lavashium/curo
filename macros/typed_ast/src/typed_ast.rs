use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericArgument,
    Ident, PathArguments, Type,
};

pub fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let name_str = ident.to_string();

    assert!(name_str.starts_with("Ast"));
    let new_name = format_ident!("Typed{}", &name_str[3..]);

    let no_ty = has_no_ty(&input.attrs);

    let expanded = match &input.data {
        Data::Struct(data) => transform_struct(&new_name, data, no_ty),
        Data::Enum(data) => transform_enum(&new_name, data, no_ty),
        Data::Union(_) => unimplemented!("Unions are not supported"),
    };

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #expanded
    }
    .into()
}

fn has_no_ty(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("typed_ast")
            && attr
                .parse_args::<Ident>()
                .map(|ident| ident == "no_ty")
                .unwrap_or(false)
    })
}

fn transform_struct(name: &Ident, data: &DataStruct, no_ty: bool) -> TokenStream {
    let mut fields = vec![];

    for field in &data.fields {
        if field.ident.as_ref().unwrap() == "span" {
            continue;
        }

        let ident = &field.ident;
        let ty = transform_type(&field.ty);
        fields.push(quote! { #ident: #ty });
    }

    if !no_ty {
        fields.push(quote! { ty: AstType });
    }

    quote! {
        pub struct #name {
            #(#fields),*
        }
    }
}

fn transform_enum(name: &Ident, data: &DataEnum, no_ty: bool) -> TokenStream {
    let mut variants = vec![];

    for var in &data.variants {
        let var_ident = &var.ident;
        match &var.fields {
            Fields::Named(fields_named) => {
                let mut fields = vec![];
                for field in &fields_named.named {
                    let ident = &field.ident;
                    let ty = transform_type(&field.ty);
                    fields.push(quote! { #ident: #ty });
                }
                if !no_ty {
                    fields.push(quote! { ty: AstType });
                }

                variants.push(quote! {
                    #var_ident { #(#fields),* }
                });
            }
            Fields::Unnamed(_) => {
                panic!("Tuple variant is not supported");
            }
            Fields::Unit => {
                if no_ty {
                    variants.push(quote! {
                        #var_ident
                    });
                } else {
                    variants.push(quote! {
                        #var_ident { ty: AstType }
                    });
                }
            }
        }
    }

    quote! {
        pub enum #name {
            #(#variants),*
        }
    }
}

fn transform_type(ty: &Type) -> TokenStream {
    match ty {
        Type::Path(type_path) => {
            let segments = &type_path.path.segments;

            if segments.len() == 1 {
                let segment = &segments[0];
                let ident_str = segment.ident.to_string();

                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if args.args.len() == 1 {
                        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                            let inner_ts = transform_type(inner_ty);

                            match ident_str.as_str() {
                                "Box" => return quote! { Box<#inner_ts> },
                                "Option" => return quote! { Option<#inner_ts> },
                                "Vec" => return quote! { Vec<#inner_ts> },
                                _ => {}
                            }
                        }
                    }
                }

                let excluded = ["AstUnaryKind", "AstBinaryKind", "AstType"];
                if excluded.contains(&ident_str.as_str()) {
                    return quote! { #ty };
                }

                if ident_str.starts_with("Ast") {
                    let new_ident = format_ident!("Typed{}", &ident_str[3..]);
                    return quote! { #new_ident };
                }

                quote! { #ty }
            } else {
                quote! { #ty }
            }
        }
        _ => quote! { #ty },
    }
}
