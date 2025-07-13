use language::*;
use super::*;

pub fn typecheck_block_item(
    item: &mut AstBlockItem,
    symbols: &mut SymbolTable,
    ctx: &mut SemanticContext<'_>,
) {
    match item {
        AstBlockItem::Declaration(decl) =>
            typecheck_declaration(decl, symbols, ctx),
        AstBlockItem::Statement(stmt) =>
            typecheck_statement(stmt, symbols, ctx),
    }
}