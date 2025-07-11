use language::*;
use super::*;

pub fn resolve_function(
    func: &mut AstFunction,
    ctx: &mut SemanticContext,
    map: &mut VariableMap,
) {
    resolve_block(func.body_mut(), ctx, map);
}
