use language::*;
use super::*;

pub fn resolve_program(
    program: &mut AstProgram,
    ctx: &mut SemanticContext,
    map: &mut VariableMap,
) {
    resolve_function(program.function_definition_mut(), ctx, map);
}

