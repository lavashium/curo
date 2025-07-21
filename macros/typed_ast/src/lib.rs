extern crate proc_macro;

use proc_macro::TokenStream;

mod typed_ast;

#[proc_macro_derive(TypedAst, attributes(typed_ast))]
pub fn derive_typed_ast(input: TokenStream) -> TokenStream {
    typed_ast::generate(input)
}
