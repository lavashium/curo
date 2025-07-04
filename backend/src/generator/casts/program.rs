use crate::asm::*;
use language::*;
use super::*;

pub trait ProgramCast {
    fn cast_program(&self, program: &TacProgram) -> AsmProgram;
}

impl<'a> ProgramCast for GeneratorCasts<'a> {
    fn cast_program(&self, program: &TacProgram) -> AsmProgram {
        let function = program.function_definition();
        let function_definition = self.cast_function(function);
        AsmProgram::new(
            function_definition
        )
    }
}