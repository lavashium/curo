use super::*;
use language::*;

pub trait ProgramTransform {
    fn transform_program(&mut self, program: &AstProgram) -> TacProgram;
}

impl<'a> ProgramTransform for GeneratorTransforms<'a> {
    fn transform_program(&mut self, program: &AstProgram) -> TacProgram {
        let function = program.function_definition();
        let function_definition = self.transform_function(function);
        TacProgram::new(function_definition)
    }
}
