use common::*;
use super::*;

impl Factory<(), TypedProgram, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext) -> () {
        for func in program.functions_mut() {
            Self::run(func, ctx)
        }
    }
}