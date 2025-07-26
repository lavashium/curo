use common::*;
use super::*;

impl Factory<(), TypedFunctionDeclaration, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(function_declaration: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext) {
        if let Some(entry) = ctx.scope.get(function_declaration.identifier()) {
            if entry.from_current_scope && !entry.has_linkage {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    function_declaration.get_span(),
                    DiagnosticKind::DuplicateDeclaration { name: function_declaration.get_identifier() },
                ));
            }
        }

        ctx.scope.insert(
            function_declaration.get_identifier(),
            IdentifierInfo {
                unique_name: function_declaration.get_identifier(),
                has_linkage: true, 
                from_current_scope: true,
            }
        );

        if let Some(body) = function_declaration.body() {
            if ctx.inside_function {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    body.get_span(),
                    DiagnosticKind::Custom("nested functions not allowed".to_string()),
                ));
            }
        }

        let old_scope = ctx.scope.clone();
        ctx.scope = copy_identifier_map(&ctx.scope);
        let function_declaration_span = function_declaration.get_span();

        for param in function_declaration.params_mut() {
            let orig_name = param.clone();
            if let Some(entry) = ctx.scope.get(&orig_name) {
                if entry.from_current_scope {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        function_declaration_span,
                        DiagnosticKind::DuplicateDeclaration { name: orig_name.clone() },
                    ));
                }
            }
            let unique_name = ctx.ctx.tempgen.temp_from(orig_name.clone());
            *param = unique_name.clone();
            ctx.scope.insert(
                orig_name,
                IdentifierInfo {
                    unique_name,
                    has_linkage: false,
                    from_current_scope: true,
                }
            );
        }

        if let Some(block) = function_declaration.body_mut() {
            ctx.inside_function = true;
            
            for item in block.block_items_mut() {
                Self::run(item, ctx);
            }
            
            ctx.inside_function = false;
        }

        ctx.scope = old_scope;
    }
}