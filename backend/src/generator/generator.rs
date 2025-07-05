use super::casts::*;
use crate::asm::*;
use constructors::constructors;
use language::*;

#[constructors]
pub struct AsmGenerator;

impl AsmGenerator {
    pub fn generate(&mut self, program: TacProgram) -> AsmProgram {
        GeneratorCasts::new(self).cast_program(&program)
    }
}
