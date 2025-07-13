use language::*;
use super::*;

pub fn resolve_block_item(
    item: &mut AstBlockItem,
    ctx: &mut SemanticContext,
    map: &mut IdentifierMap,
) {
    match item {
        AstBlockItem::Statement(stmt) => resolve_statement(stmt, ctx, map),
        AstBlockItem::Declaration(decl) => match decl {
            AstDeclaration::VarDecl(v) => resolve_variable_declaration(v, ctx, map),
            AstDeclaration::FunDecl(f) => {
                resolve_function_declaration(f, ctx, map)
            }
        },
    }
}