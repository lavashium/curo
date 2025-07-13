use language::*;
use super::*;

pub fn declare_identifier(
    name: &str,
    has_linkage: bool,
    span: Span,
    ctx: &mut SemanticContext<'_>,
    map: &mut IdentifierMap,
) -> bool {
    if let Some(existing) = map.get(name) {
        if existing.from_current_scope {
            if !(has_linkage && existing.has_linkage) {
                push_error(
                    ctx,
                    span,
                    DiagnosticKind::DuplicateDeclaration {
                        name: name.to_string(),
                    },
                );
                return false;
            }
        }
    }

    map.insert(
        name.to_string(),
        IdentifierInfo {
            unique_name: ctx.temp_gen.temp_from(name.to_string()),
            has_linkage,
            from_current_scope: true,
        },
    );
    true
}