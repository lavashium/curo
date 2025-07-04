mod program;
mod function;
mod instruction;

pub use program::*;
pub use function::*;
pub use instruction::*;

use super::AsmGenerator;

#[allow(dead_code)]
pub struct GeneratorCasts<'a> {
    generator: &'a mut AsmGenerator,
}

impl<'a> GeneratorCasts<'a> {
    pub fn new(generator: &'a mut AsmGenerator) -> Self {
        Self {
            generator,
        }
    }
}