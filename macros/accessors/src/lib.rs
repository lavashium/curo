use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_attribute]
pub fn accessors(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => &named_fields.named,
            _ => {
                return syn::Error::new_spanned(&input, "Expected named fields")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "Expected a struct")
                .to_compile_error()
                .into();
        }
    };

    let mut methods = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        let getter_ref = quote! {
            pub fn #field_name(&self) -> &#field_ty {
                &self.#field_name
            }
        };

        let getter_owned_name = format_ident!("get_{}", field_name);
        let getter_owned = quote! {
            pub fn #getter_owned_name(&self) -> #field_ty
            where
                #field_ty: Clone,
            {
                self.#field_name.clone()
            }
        };

        let getter_mut_name = format_ident!("{}_mut", field_name);
        let getter_mut = quote! {
            pub fn #getter_mut_name(&mut self) -> &mut #field_ty {
                &mut self.#field_name
            }
        };

        let setter_name = format_ident!("set_{}", field_name);
        let setter = quote! {
            pub fn #setter_name(&mut self, value: #field_ty) {
                self.#field_name = value;
            }
        };

        let into_getter_name = format_ident!("into_{}", field_name);
        let into_getter = quote! {
            pub fn #into_getter_name(self) -> #field_ty {
                self.#field_name
            }
        };

        methods.push(quote! {
            #getter_ref
            #getter_owned
            #getter_mut
            #setter
            #into_getter
        });
    }

    let expanded = quote! {
        #input

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}
