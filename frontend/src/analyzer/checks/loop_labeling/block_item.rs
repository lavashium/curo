use language::*;
use super::*;

pub fn label_block_item(
    item: &mut AstBlockItem,
    ctx: &mut SemanticContext,
    current_loop: Option<String>,
) {
    match item {
        AstBlockItem::Statement(stmt) => label_statement(stmt, ctx, current_loop),
        AstBlockItem::Declaration(_) => {}
    }
}