use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Meta, MetaNameValue, Expr, ExprLit, Lit};

#[proc_macro_attribute]
pub fn debug(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let label = if attr.is_empty() {
        input_fn.sig.ident.to_string()
    } else {
        let meta: Meta = syn::parse(attr).expect("Expected `label = \"...\"`");
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
            if path.is_ident("label") {
                if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = value {
                    s.value()
                } else {
                    panic!("Expected string literal for label");
                }
            } else {
                panic!("Unknown attribute key, expected `label`");
            }
        } else {
            panic!("Invalid attribute format, expected `label = \"...\"`");
        }
    };

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let attrs = &input_fn.attrs;
    let block = &input_fn.block;

let debug_block = quote! {
        println!("[DEBUG] entering  `{}`", #label);
        let start = std::time::Instant::now();
        let result = (|| #block)();
        let duration = start.elapsed();
        println!("[DEBUG] executing `{}` took {:?}", #label, duration);
        result
    };


    let output = quote! {
        #(#attrs)*
        #vis #sig {
            #[cfg(debug_assertions)]
            {
                #debug_block
            }
            #[cfg(not(debug_assertions))]
            {
                #block
            }
        }
    };

    output.into()
}
