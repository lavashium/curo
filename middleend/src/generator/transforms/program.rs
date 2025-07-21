use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_program(&mut self, program: &mut TypedProgram) -> TacProgram {
        <Self as Factory<TacProgram, Self, TypedProgram>>::run(self, program)
    }
}

impl<'scp, 'ctx> Factory<TacProgram, Self, TypedProgram> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, program: &mut TypedProgram) -> TacProgram {
        let functions = program.functions_mut();
        let tac_functions = functions
            .iter_mut()
            .filter(|f| f.body().is_some()) 
            .map(|f| driver.transform_function_declaration(f))
            .collect();

        TacProgram::new(tac_functions)
    }
}