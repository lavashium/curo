use language::*;
use super::*;

pub fn label_block(
    block: &mut AstBlock,
    ctx: &mut SemanticContext,
    current_loop: &Option<String>,
) {
    for item in block.block_items_mut() {
        label_block_item(item, ctx, current_loop.clone());
    }
}