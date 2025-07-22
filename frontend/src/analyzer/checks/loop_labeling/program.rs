use common::*;
use super::*;

impl LoopLabeling {
    pub fn label_program(program: &mut TypedProgram, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedProgram, AnalyzerContext<'_, '_>>>::run(program, ctx)
    }
}

impl Factory<(), TypedProgram, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext) -> () {
        for func in program.functions_mut() {
            Self::label_function_declaration(func, ctx)
        }
    }
}