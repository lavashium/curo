use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedFunctionDeclaration> for IdentifierResolution<'scp, 'ctx> {
    fn run(declaration: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match ctx.global_scope {
            true  => { GlobalFunctionDeclarationResolver::run(declaration, ctx) }
            false => { LocalFunctionDeclarationResolver::run(declaration, ctx) }
        }
    }
}

pub struct GlobalFunctionDeclarationResolver<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for GlobalFunctionDeclarationResolver<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedFunctionDeclaration> for GlobalFunctionDeclarationResolver<'scp, 'ctx> {
    fn run(declaration: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        if let Some(entry) = ctx.scope.get(declaration.identifier()) {
            if entry.from_current_scope && !entry.has_linkage {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    declaration.get_span(),
                    DiagnosticKind::Semantic(SemanticError::DuplicateDeclaration {
                        name: declaration.get_identifier()
                    })
                ))
            }
        }

        ctx.scope.insert(
            declaration.get_identifier(),
            IdentifierInfo {
                unique_name: declaration.get_identifier(),
                has_linkage: true, 
                from_current_scope: true,
            }
        );

        let old_scope = ctx.scope.clone();
        ctx.scope = copy_identifier_map(&ctx.scope);

        let old_global_scope = ctx.global_scope;
        let old_inside_function = ctx.inside_function;

        ctx.global_scope = false;
        ctx.inside_function = true;

        let declaration_span = declaration.get_span();

        for param in declaration.params_mut() {
            let orig_name = param.clone();
            if let Some(entry) = ctx.scope.get(&orig_name) {
                if entry.from_current_scope {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        declaration_span,
                        DiagnosticKind::Semantic(SemanticError::DuplicateDeclaration { name: orig_name.clone() }),
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

        if let Some(block) = declaration.body_mut() {
            ctx.inside_function = true;
            
            for item in block.block_items_mut() {
                IdentifierResolution::run(item, ctx);
            }
            
            ctx.inside_function = false;
        }

        ctx.global_scope = old_global_scope;
        ctx.inside_function = old_inside_function;
        ctx.scope = old_scope;       
    }
}

pub struct LocalFunctionDeclarationResolver<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for LocalFunctionDeclarationResolver<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedFunctionDeclaration> for LocalFunctionDeclarationResolver<'scp, 'ctx> {
    fn run(declaration: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        if declaration.body().is_some() {
            ctx.ctx.diagnostics.push(Diagnostic::error(
                declaration.get_span(),
                DiagnosticKind::Semantic(SemanticError::NestedFunctionDefinition),
            ));
        }
        
        if declaration.storage_class() == &Some(AstStorageClass::Static) {
            ctx.ctx.diagnostics.push(Diagnostic::error(
                declaration.get_span(),
                DiagnosticKind::Semantic(SemanticError::InvalidStorageSpecifier),
            ));
        }

        GlobalFunctionDeclarationResolver::run(declaration, ctx)
    }
}