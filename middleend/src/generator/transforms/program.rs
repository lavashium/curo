use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_program(&mut self, program: &mut AstProgram) -> TacProgram {
        <Self as Factory<TacProgram, Self, AstProgram>>::run(self, program)
    }
}

impl<'scp, 'ctx> Factory<TacProgram, Self, AstProgram> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, program: &mut AstProgram) -> TacProgram {
        let functions = program.functions_mut();
        let tac_functions = functions
            .iter_mut()
            .filter(|f| f.body().is_some()) 
            .map(|f| driver.transform_function_declaration(f))
            .collect();

        TacProgram::new(tac_functions)
    }
}