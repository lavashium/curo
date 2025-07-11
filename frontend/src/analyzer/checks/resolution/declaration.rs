use language::*;
use super::*;

pub fn resolve_declaration(
    decl: &mut AstDeclaration,
    ctx: &mut SemanticContext,
    map: &mut VariableMap,
) {
    let name = decl.name().clone();
    if let Some(existing) = map.get(&name) {
        if existing.from_current_block {
            ctx.diagnostics_mut().push(Diagnostic::error(
                decl.get_span(),
                DiagnosticKind::DuplicateVariableDeclaration { name: name.clone() },
            ));
        }
    }
    let unique = ctx.temp_gen_mut().temp_from(name.clone());
    map.insert(name.clone(), VariableInfo { unique_name: unique.clone(), from_current_block: true });
    decl.set_name(unique);

    if let Some(init_expr) = decl.init_mut() {
        resolve_expression(init_expr, ctx, map);
    }
}
