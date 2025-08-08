use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedFunctionDeclaration> for TypeCheck<'scp, 'ctx> {
    fn run(decl: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        let name = decl.identifier().clone();
        let params = decl.get_params();
        let span = decl.get_span();
        let fun_type = AstType::FunType(params.len());
        let has_body = decl.body().is_some();
        let storage = decl.storage_class();
        let global_linkage = storage != &Some(AstStorageClass::Static);

        let (defined, global_prev) = if let Some(prev) = ctx.ctx.symtable.get(&name) {
            if prev.ty != fun_type {
                ctx.ctx.diagnostics.push(
                    Diagnostic::error(
                        span,
                        DiagnosticKind::Semantic(SemanticError::ConflictingDeclarations { name: name.clone() })
                    )
                );
            }
            match &prev.attrs {
                IdentifierAttrs::FunAttr { defined: prev_def, global: prev_glob, .. } => {

                    if *prev_glob && storage == &Some(AstStorageClass::Static) {
                        ctx.ctx.diagnostics.push(
                            Diagnostic::error(
                                span,
                                DiagnosticKind::Semantic(SemanticError::ConflictingStorageSpecifiers)
                            )
                        );
                    }

                    if *prev_def && has_body {
                        ctx.ctx.diagnostics.push(
                            Diagnostic::error(
                                span,
                                DiagnosticKind::Semantic(SemanticError::DuplicateDeclaration { name: name.clone() })
                            )
                        );
                    }
                    ( *prev_def || has_body, *prev_glob )
                }
                _ => panic!("Internal error: symbol has function type but not attrs"),
            }
        } else {
            (has_body, global_linkage)
        };

        ctx.ctx.symtable.add_fun(name.clone(), fun_type.clone(), global_prev, defined);

        if let Some(body) = decl.body_mut() {
            ctx.global_scope = false;
            ctx.inside_function = true;

            for p in params {
                ctx.ctx.symtable.add_automatic_var(p.clone(), AstType::Int);
            }

            Self::run(body, ctx);

            ctx.global_scope = true;
            ctx.inside_function = false;
        }
    }
}