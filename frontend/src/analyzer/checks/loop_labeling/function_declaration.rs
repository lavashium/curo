use common::*;
use super::*;

impl LoopLabeling {
    pub fn label_function_declaration(function_declaration: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedFunctionDeclaration, AnalyzerContext<'_, '_>>>::run(function_declaration, ctx)
    }
}

impl Factory<(), TypedFunctionDeclaration, AnalyzerContext<'_, '_>> for LoopLabeling {
    fn run(function_declaration: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext) -> () {
        if let Some(block) = function_declaration.body_mut() {
            LoopLabeling::label_block(block, ctx)
        }
    }
}