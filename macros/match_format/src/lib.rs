
use proc_macro::TokenStream;
use quote::{quote};
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Expr, LitStr, Pat, Token,
};

struct Input {
    expr: Expr,
    arms: Vec<Arm>,
}

struct Arm {
    pattern: Pat,
    lines: Vec<(LitStr, Vec<Expr>)>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let content;
        braced!(content in input);

        let mut arms = Vec::new();
        while !content.is_empty() {
            arms.push(content.parse()?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(Input { expr, arms })
    }
}

impl Parse for Arm {
    fn parse(input: ParseStream) -> Result<Self> {
        let pattern = Pat::parse_single(input)?;
        input.parse::<Token![=>]>()?;

        let lines = if input.peek(LitStr) {
            let fmt: LitStr = input.parse()?;
            input.parse::<Token![,]>()?;
            let content;
            syn::bracketed!(content in input);
            let args = content.parse_terminated(Expr::parse, Token![,])?;
            vec![(fmt, args.into_iter().collect())]
        } else {
            let content;
            braced!(content in input);
            let mut lines = Vec::new();
            while !content.is_empty() {
                let fmt: LitStr = content.parse()?;
                content.parse::<Token![,]>()?;
                let args_content;
                syn::bracketed!(args_content in content);
                let args = args_content.parse_terminated(Expr::parse, Token![,])?;
                content.parse::<Token![,]>()?;
                lines.push((fmt, args.into_iter().collect()));
            }
            lines
        };

        Ok(Arm { pattern, lines })
    }
}


#[proc_macro]
pub fn emit_instruction(input: TokenStream) -> TokenStream {
    let Input { expr, arms } = parse_macro_input!(input as Input);

    let arms_tokens = arms.into_iter().map(|Arm { pattern, lines }| {
        let format_lines = lines.into_iter().map(|(fmt, args)| {
            quote! {
                {
                    let formatted = format!(#fmt, #(#args.to_asm()),*);
                    instr_lines.push(format!("    {}", formatted));
                }
            }
        });

        quote! {
            #pattern => {
                let mut instr_lines = Vec::new();
                #(#format_lines)*
                instr_lines.join("\n") + "\n"
            }
        }
    });

    let expanded = quote! {
        match #expr {
            #(#arms_tokens),*,
            _ => String::new(),
        }
    };

    expanded.into()
}
