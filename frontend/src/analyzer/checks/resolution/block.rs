use language::*;
use super::*;

pub fn resolve_block(
    block: &mut AstBlock,
    ctx: &mut SemanticContext,
    parent_map: &mut VariableMap,
) {
    let mut local_map = copy_variable_map(parent_map);
    for item in block.block_items_mut() {
        resolve_block_item(item, ctx, &mut local_map);
    }
}
