use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedDeclaration> for LoopLabeling<'scp, 'ctx> {
    fn run(declaration: &mut TypedDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) -> () {
        match declaration {
            TypedDeclaration::FunDecl(fun) => Self::run(fun, ctx),
            TypedDeclaration::VarDecl(_) => ()
        }
    }
}