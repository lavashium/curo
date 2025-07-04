use crate::asm::*;
use language::*;
use super::casts::*;
use constructors::constructors;

#[constructors]
pub struct AsmGenerator;

impl AsmGenerator {
    pub fn generate(&mut self, program: TacProgram) -> AsmProgram {
        GeneratorCasts::new(self).cast_program(&program)
    }
}