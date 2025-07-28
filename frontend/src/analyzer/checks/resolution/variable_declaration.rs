use common::*;
use super::*;

impl Factory<(), TypedVariableDeclaration, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(decl: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext) {
        if let Some(entry) = ctx.scope.get(decl.identifier()) {
            if *entry.from_current_scope() {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    decl.get_span(),
                    DiagnosticKind::Semantic(SemanticError::DuplicateDeclaration { name: decl.get_identifier() }),
                ));
            }
        }
        
        let unique_name = ctx.ctx.tempgen.temp_from(decl.get_identifier().clone());
        ctx.scope.insert(
            decl.get_identifier().clone(),
            IdentifierInfo {
                unique_name: unique_name.clone(),
                has_linkage: false,
                from_current_scope: true, 
            }
        );
        decl.set_identifier(unique_name);
        
        Self::run_option(decl.init_mut(), ctx);
    }
}