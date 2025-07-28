use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Meta, MetaNameValue, Expr, ExprLit, Lit};

#[proc_macro_attribute]
pub fn zawarudo(attr: TokenStream, item: TokenStream) -> TokenStream {
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

    let zawarudo_block = quote! {
        const YELLOW_BOLD: &str = "\x1b[1;33m";
        const BLUE_BOLD:   &str = "\x1b[1;34m";
        const RESET:       &str = "\x1b[0m";

        eprintln!(
            "{}[ZA WARUDO!]{} TOKI WO TOMARE!    {}{:<18}{} froze time!",
            YELLOW_BOLD,
            RESET,
            BLUE_BOLD,
            #label,
            RESET
        );

        let timer_start = std::time::Instant::now();
        let result = {
            #block
        };
        let elapsed = timer_start.elapsed();

        eprintln!(
            "{}[ZA WARUDO!]{} TOKI WA UGOKIDASU! {}{:<18}{} executed in {}{:?}{}",
            YELLOW_BOLD,
            RESET,
            BLUE_BOLD,
            #label,
            RESET,
            BLUE_BOLD,
            elapsed,
            RESET
        );

        result
    };


    let output = quote! {
        #(#attrs)*
        #vis #sig {
            #[cfg(feature = "zawarudo")]
            {
                #zawarudo_block
            }
            #[cfg(not(feature = "zawarudo"))]
            {
                #block
            }
        }
    };

    output.into()
}
