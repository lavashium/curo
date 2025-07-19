use super::*;
use crate::asm::*;
use language::*;

pub trait ProgramCast {
    fn cast_program(&self, program: &TacProgram) -> AsmProgram;
}

impl<'a> ProgramCast for GeneratorCasts<'a> {
    fn cast_program(&self, program: &TacProgram) -> AsmProgram {
        let function_definitions = program.function_definitions()
            .iter()
            .map(|f| self.cast_function(f))
            .collect();
        AsmProgram::new(function_definitions)
    }
}