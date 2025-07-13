use language::*;
use super::*;

pub fn typecheck_declaration(
    decl: &mut AstDeclaration,
    symbols: &mut SymbolTable,
    ctx: &mut SemanticContext<'_>,
) {
    match decl {
        AstDeclaration::VarDecl(v) => {
            symbols.add_variable(v.identifier(), Span::default(), ctx);
            if let Some(init) = v.init_mut() {
                typecheck_expression(init, symbols, ctx);
            }
        }
        AstDeclaration::FunDecl(f) => {
            typecheck_function_declaration(f, symbols, ctx);
        }
    }
}