use common::*;
use super::*;

impl IdentifierResolution {
    pub fn resolve_variable_declaration(decl: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedVariableDeclaration, AnalyzerContext<'_, '_>>>::run(decl, ctx)
    }
}

impl Factory<(), TypedVariableDeclaration, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(decl: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext) {
        let span = decl.init().as_ref().map(|e| e.get_span()).unwrap_or_default();
        if let Some(unique_name) = ctx.declare_identifier(decl.identifier(), false, span) {
            decl.set_identifier(unique_name);
            if let Some(init) = decl.init_mut() {
                Self::resolve_expression(init, ctx);
            }
        }
    }
}