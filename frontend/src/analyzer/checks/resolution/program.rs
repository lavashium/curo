use common::*;
use super::*;

impl IdentifierResolution {
    pub fn resolve_program(program: &mut TypedProgram, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedProgram, AnalyzerContext<'_, '_>>>::run(program, ctx)
    }
}

impl Factory<(), TypedProgram, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext) -> () {
        for func in program.functions_mut() {
            Self::resolve_function_declaration(func, ctx)
        }
    }
}