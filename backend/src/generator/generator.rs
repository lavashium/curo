use crate::asm::*;
use language::*;
use super::casts::*;

pub struct AsmGenerator;

impl AsmGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&mut self, program: TacProgram) -> AsmProgram {
        GeneratorCasts::new(self).cast_program(&program)
    }
}