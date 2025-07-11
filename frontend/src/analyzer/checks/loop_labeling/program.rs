use language::*;
use super::*;

pub fn label_program(
    program: &mut AstProgram,
    ctx: &mut SemanticContext,
    current_loop: Option<String>,
) {
    label_function(program.function_definition_mut(), ctx, current_loop);
}