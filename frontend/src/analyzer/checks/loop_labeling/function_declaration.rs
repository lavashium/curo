use language::*;
use super::*;

pub fn label_function_declaration(
    func: &mut AstFunctionDeclaration,
    ctx: &mut SemanticContext,
    current_loop: &Option<String>,
) {
    if let Some(body) = func.body_mut() {
        label_block(body, ctx, current_loop);
    }
}