use common::*;
use super::*;
use super::function_declaration::*;
use super::variable_declaration::*;

impl<'scp, 'ctx> Factory<(), TypedDeclaration> for IdentifierResolution<'scp, 'ctx> {
    fn run(declaration: &mut TypedDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match ctx.global_scope {
            true  => { GlobalDeclarationResolver::run(declaration, ctx) }
            false => { LocalDeclarationResolver::run(declaration, ctx) }
        }
    }
}

pub struct GlobalDeclarationResolver<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for GlobalDeclarationResolver<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedDeclaration> for GlobalDeclarationResolver<'scp, 'ctx> {
    fn run(declaration: &mut TypedDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match declaration {
            TypedDeclaration::FunDecl(fun) => {
                GlobalFunctionDeclarationResolver::run(fun, ctx);
            }
            TypedDeclaration::VarDecl(var) => {
                GlobalVariableDeclarationResolver::run(var, ctx);
            }
        }
    }
}

pub struct LocalDeclarationResolver<'scp, 'ctx> {
    _driver: PhantomData<AnalyzerContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for LocalDeclarationResolver<'scp, 'ctx> {
    type Context = AnalyzerContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<(), TypedDeclaration> for LocalDeclarationResolver<'scp, 'ctx> {
    fn run(declaration: &mut TypedDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match declaration {
            TypedDeclaration::FunDecl(fun) => {
                LocalFunctionDeclarationResolver::run(fun, ctx);
            }
            TypedDeclaration::VarDecl(var) => {
                LocalVariableDeclarationResolver::run(var, ctx);
            }
        }
    }
}