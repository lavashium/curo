use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedFunctionDeclaration> for LoopLabeling<'scp, 'ctx> {
    fn run(function_declaration: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext<'scp, 'ctx>) -> () {
        LoopLabeling::run_option(function_declaration.body_mut(), ctx);
    }
}