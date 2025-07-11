use language::*;
use super::*;

pub fn label_function(
    func: &mut AstFunction,
    ctx: &mut SemanticContext,
    current_loop: Option<String>,
) {
    label_block(func.body_mut(), ctx, current_loop);
}