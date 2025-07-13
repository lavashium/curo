use language::*;
use super::*;

pub fn typecheck_function_declaration(
    func: &mut AstFunctionDeclaration,
    symbols: &mut SymbolTable,
    ctx: &mut SemanticContext<'_>,
) {
    let name = func.identifier();
    let param_list = func.params().to_vec();
    let has_body = func.body().is_some();
    let span = Span::default();

    symbols.add_function_decl(name, func.params().len(), has_body, span, ctx);

    if let Some(body) = func.body_mut() {
        symbols.push_scope();
        
        for p in &param_list {
            symbols.add_variable(p, Span::default(), ctx);
        }
        
        typecheck_block(body, symbols, ctx);
        symbols.pop_scope();
    }
}