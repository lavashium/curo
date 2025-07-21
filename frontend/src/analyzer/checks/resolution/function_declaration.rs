use common::*;
use super::*;

impl IdentifierResolution {
    pub fn resolve_function_declaration(function_declaration: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedFunctionDeclaration, AnalyzerContext<'_, '_>>>::run(function_declaration, ctx)
    }
}

impl Factory<(), TypedFunctionDeclaration, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(func: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext) {
        if ctx.inside_function && func.body().is_some() {
            ctx.ctx.diagnostics.push(Diagnostic::error(
                func.get_span(),
                DiagnosticKind::NestedFunctionDefinition,
            ));
            return;
        }

        if func.body().is_none() {
            return;
        }

        ctx.inside_function = true;
        ctx.push_scope(true);
        
        for param in func.params_mut() {
            if let Some(unique_name) = ctx.declare_identifier(param, false, Span::default()) {
                *param = unique_name;
            }
        }

        if let Some(body) = func.body_mut() {
            Self::resolve_block(body, ctx);
        }
        
        ctx.pop_scope();
        ctx.inside_function = false;
    }
}
