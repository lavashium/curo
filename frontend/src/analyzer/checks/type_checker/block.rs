use language::*;
use super::*;

pub fn typecheck_block(
    block: &mut AstBlock,
    symbols: &mut SymbolTable,
    ctx: &mut SemanticContext<'_>,
) {
    symbols.push_scope();
    for item in block.block_items_mut() {
        typecheck_block_item(item, symbols, ctx);
    }
    symbols.pop_scope();
}