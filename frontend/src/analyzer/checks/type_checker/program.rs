use language::*;
use super::*;

pub fn typecheck_program(
    ast: &mut AstProgram,
    symbols: &mut SymbolTable,
    ctx: &mut SemanticContext<'_>,
) {
    for func in ast.functions_mut() {
        typecheck_function_declaration(func, symbols, ctx);
    }
}