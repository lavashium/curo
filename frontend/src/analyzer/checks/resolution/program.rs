use language::*;
use super::*;

pub fn resolve_program(
    program: &mut AstProgram,
    ctx: &mut SemanticContext,
    map: &mut IdentifierMap,
) {
    for func in program.functions_mut() {
        resolve_function_declaration(func, ctx, map);
    }
}