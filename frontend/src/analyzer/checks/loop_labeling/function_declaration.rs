use common::*;
use super::*;

impl Factory<(), TypedFunctionDeclaration, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(function_declaration: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext) -> () {
        LoopLabeling::run_option(function_declaration.body_mut(), ctx);
    }
}