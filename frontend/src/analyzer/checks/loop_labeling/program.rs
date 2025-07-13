use language::*;
use super::*;

pub fn label_program(
    program: &mut AstProgram,
    ctx: &mut SemanticContext,
    current_loop: &Option<String>,
) {
    for function in program.functions_mut() {
        label_function_declaration(function, ctx, current_loop);
    }
}