use language::*;
use super::*;

pub fn resolve_block_item(
    item: &mut AstBlockItem,
    ctx: &mut SemanticContext,
    map: &mut VariableMap,
) {
    match item {
        AstBlockItem::Declaration(decl) => resolve_declaration(decl, ctx, map),
        AstBlockItem::Statement(stmt) => resolve_statement(stmt, ctx, map),
    }
}
