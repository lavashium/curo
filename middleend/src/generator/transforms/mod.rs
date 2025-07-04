mod program;
mod function;
mod statement;
mod expression;

pub use program::*;
pub use function::*;
pub use statement::*;
pub use expression::*;

use super::TacGenerator;

pub struct GeneratorTransforms<'a> {
    generator: &'a mut TacGenerator
}

impl<'a> GeneratorTransforms<'a> {
    pub fn new(generator: &'a mut TacGenerator) -> Self {
        Self {
            generator,
        }
    }    
}