use common::*;
use super::*;

impl Factory<(), TypedProgram, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext) {
        for func in program.functions_mut() {
            TypeCheck::run(func, ctx);
        }
    }
}