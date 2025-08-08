use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedVariableDeclaration> for TypeCheck<'scp, 'ctx> {
    fn run(declaration: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match ctx.global_scope {
            true  => { GlobalVariableDeclarationTypeCheck::run(declaration, ctx) }
            false => { LocalVariableDeclarationTypeCheck::run(declaration, ctx) }
        }
    }
}

pub struct GlobalVariableDeclarationTypeCheck<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for GlobalVariableDeclarationTypeCheck<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedVariableDeclaration> for GlobalVariableDeclarationTypeCheck<'scp, 'ctx> {
    fn run(declaration: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        let span = declaration.get_span();
        let name = declaration.identifier().clone();
        let storage_class = declaration.storage_class().clone();
        
        let current_init = match declaration.init() {
            Some(TypedExpression::Constant { constant, .. }) => 
                InitialValue::Initial(constant.clone()),
            None if storage_class == Some(AstStorageClass::Extern) => 
                InitialValue::NoInitializer,
            None => 
                InitialValue::Tentative,
            Some(_) => {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    span,
                    DiagnosticKind::Semantic(SemanticError::NonConstInitializer),
                ));
                InitialValue::NoInitializer
            }
        };
        
        let current_global = storage_class != Some(AstStorageClass::Static);
        
        match ctx.ctx.symtable.get(&name) {
            Some(symbol) => {
                match &symbol.attrs {
                    IdentifierAttrs::StaticAttr { init: prev_init, global: prev_global } => {
                        if symbol.ty != AstType::Int {
                            ctx.ctx.diagnostics.push(Diagnostic::error(
                                span,
                                DiagnosticKind::Semantic(SemanticError::FunctionRedefinition { name: name.clone() }),
                            ));
                            return;
                        }
                        
                        let final_global = if storage_class == Some(AstStorageClass::Extern) {
                            *prev_global
                        } else if current_global == *prev_global {
                            current_global
                        } else {
                            ctx.ctx.diagnostics.push(Diagnostic::error(
                                span,
                                DiagnosticKind::Semantic(SemanticError::InvalidStorageSpecifier),
                            ));
                            current_global
                        };
                        
                        let final_init = match (prev_init, &current_init) {
                            (InitialValue::Initial(_), InitialValue::Initial(_)) => {
                                ctx.ctx.diagnostics.push(Diagnostic::error(
                                    span,
                                    DiagnosticKind::Semantic(SemanticError::DuplicateDeclaration { name: name.clone() }),
                                ));
                                prev_init.clone()
                            }
                            (InitialValue::Initial(i), _) => InitialValue::Initial(i.clone()),
                            (_, InitialValue::Initial(i)) => InitialValue::Initial(i.clone()),
                            (InitialValue::Tentative, _) => {
                                match &current_init {
                                    InitialValue::Tentative | InitialValue::NoInitializer => 
                                        InitialValue::Tentative,
                                    _ => current_init.clone() // Handled by earlier arms
                                }
                            }
                            (InitialValue::NoInitializer, current) => current.clone(),
                        };
                        
                        ctx.ctx.symtable.add_static_var(
                            name,
                            AstType::Int,
                            final_global,
                            final_init
                        );
                    }
                    _ => {
                        ctx.ctx.diagnostics.push(Diagnostic::error(
                            span,
                            DiagnosticKind::Semantic(SemanticError::ConflictingDeclarations { name }),
                        ));
                    }
                }
            }
            None => {
                ctx.ctx.symtable.add_static_var(
                    name,
                    AstType::Int,
                    current_global,
                    current_init
                );
            }
        }
    }
}
pub struct LocalVariableDeclarationTypeCheck<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for LocalVariableDeclarationTypeCheck<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedVariableDeclaration> for LocalVariableDeclarationTypeCheck<'scp, 'ctx> {
    fn run(decl: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        let name = decl.get_identifier().clone();
        let storage = decl.storage_class();
        let span = decl.get_span();

        match storage {
            Some(AstStorageClass::Extern) => {
                if decl.init().is_some() {
                    ctx.ctx.diagnostics.push(
                        Diagnostic::error(span, DiagnosticKind::Semantic(SemanticError::InvalidStorageSpecifier))
                    );
                }
                if let Some(prev) = ctx.ctx.symtable.get(&name) {
                    if prev.ty != AstType::Int {
                        ctx.ctx.diagnostics.push(
                            Diagnostic::error(span, DiagnosticKind::Semantic(SemanticError::TypeMismatch {
                                expected: AstType::Int, 
                                found: prev.ty.clone() 
                            }))
                        );
                    }
                } else {
                    ctx.ctx.symtable.add_static_var(name.clone(), AstType::Int, true, InitialValue::NoInitializer);
                }
            }
            Some(AstStorageClass::Static) => {
                let init = match decl.init() {
                    Some(TypedExpression::Constant { constant, .. }) => InitialValue::Initial(constant.clone()),
                    None => InitialValue::Initial("0".to_string()),
                    Some(_) => {
                        ctx.ctx.diagnostics.push(
                            Diagnostic::error(span, DiagnosticKind::Semantic(SemanticError::NonConstInitializer))
                        );
                        InitialValue::NoInitializer
                    }
                };
                ctx.ctx.symtable.add_static_var(name.clone(), AstType::Int, false, init);
            }
            None => {
                ctx.ctx.symtable.add_automatic_var(name.clone(), AstType::Int);
                TypeCheck::run_option(decl.init_mut(), ctx);
            }
        }
    }
}
