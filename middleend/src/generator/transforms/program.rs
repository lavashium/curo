use super::*;
use language::*;
use common::*;

impl Factory<TacProgram, TypedProgram, TacGenContext<'_, '_>> for GeneratorTransforms {
    fn run(program: &mut TypedProgram, ctx: &mut TacGenContext) -> TacProgram {
        let functions = program.functions_mut();
        let tac_functions = functions
            .iter_mut()
            .filter(|f| f.body().is_some()) 
            .map(|f| Self::run(f, ctx))
            .collect();

        TacProgram::new(tac_functions)
    }
}