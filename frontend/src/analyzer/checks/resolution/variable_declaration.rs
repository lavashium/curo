use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedVariableDeclaration> for IdentifierResolution<'scp, 'ctx> {
    fn run(declaration: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match ctx.global_scope {
            true  => { GlobalVariableDeclarationResolver::run(declaration, ctx) }
            false => { LocalVariableDeclarationResolver::run(declaration, ctx) }
        }
    }
}

pub struct GlobalVariableDeclarationResolver<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for GlobalVariableDeclarationResolver<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedVariableDeclaration> for GlobalVariableDeclarationResolver<'scp, 'ctx> {
    fn run(declaration: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        ctx.scope.insert(
            declaration.get_identifier(),
            IdentifierInfo {
                unique_name: declaration.get_identifier(),
                has_linkage: false,
                from_current_scope: true, 
            }
        );
    }
}

pub struct LocalVariableDeclarationResolver<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for LocalVariableDeclarationResolver<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedVariableDeclaration> for LocalVariableDeclarationResolver<'scp, 'ctx> {
    fn run(declaration: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        if let Some(entry) = ctx.scope.get(declaration.identifier()) {
            if entry.from_current_scope {
                if !(entry.has_linkage && declaration.storage_class() == &Some(AstStorageClass::Extern)) {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        declaration.get_span(),
                        DiagnosticKind::Semantic(SemanticError::DuplicateDeclaration {
                            name: declaration.get_identifier()
                        })
                    ));
                }
            }
        }

        let orig_name = declaration.get_identifier();

        let unique_name = if declaration.storage_class() == &Some(AstStorageClass::Extern) {
            orig_name.clone()
        } else {
            ctx.ctx.tempgen.temp_from(orig_name.clone())
        };

        declaration.set_identifier(unique_name.clone());
        
        ctx.scope.insert(
            orig_name,
            IdentifierInfo {
                unique_name,
                has_linkage: declaration.storage_class() == &Some(AstStorageClass::Extern),
                from_current_scope: true,
            }
        );

        IdentifierResolution::run_option(declaration.init_mut(), ctx);
    }
}