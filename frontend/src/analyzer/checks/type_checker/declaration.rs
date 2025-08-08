use common::*;
use super::*;
use super::variable_declaration::*;

impl<'scp, 'ctx> Factory<(), TypedDeclaration> for TypeCheck<'scp, 'ctx> {
    fn run(declaration: &mut TypedDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match ctx.global_scope {
            true  => { GlobalDeclarationTypeCheck::run(declaration, ctx) }
            false => { LocalDeclarationTypeCheck::run(declaration, ctx) }
        }
    }
}

pub struct GlobalDeclarationTypeCheck<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for GlobalDeclarationTypeCheck<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedDeclaration> for GlobalDeclarationTypeCheck<'scp, 'ctx> {
    fn run(declaration: &mut TypedDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match declaration {
            TypedDeclaration::FunDecl(fun) => {
                TypeCheck::run(fun, ctx);
            }
            TypedDeclaration::VarDecl(var) => {
                GlobalVariableDeclarationTypeCheck::run(var, ctx);
            }
        }
    }
}

pub struct LocalDeclarationTypeCheck<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for LocalDeclarationTypeCheck<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedDeclaration> for LocalDeclarationTypeCheck<'scp, 'ctx> {
    fn run(declaration: &mut TypedDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match declaration {
            TypedDeclaration::FunDecl(fun) => {
                TypeCheck::run(fun, ctx);
            }
            TypedDeclaration::VarDecl(var) => {
                LocalVariableDeclarationTypeCheck::run(var, ctx);
            }
        }
    }
}