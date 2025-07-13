use language::*;
use super::*;

pub fn resolve_variable_declaration(
    decl: &mut AstVariableDeclaration,
    ctx: &mut SemanticContext,
    map: &mut IdentifierMap,
) {
    let span = decl
        .init()
        .as_ref()
        .map(|e| e.get_span())
        .unwrap_or_default();

    let has_linkage = false;
    let ok = declare_identifier(decl.identifier(), has_linkage, span, ctx, map);

    if let Some(init) = decl.init_mut() {
        resolve_expression(init, ctx, map);
    }

    if !ok {

    }
}