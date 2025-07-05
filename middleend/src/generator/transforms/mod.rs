mod expression;
mod function;
mod program;
mod statement;

pub use expression::*;
pub use function::*;
pub use program::*;
pub use statement::*;

use super::TacGenerator;

pub struct GeneratorTransforms<'a> {
    generator: &'a mut TacGenerator,
}

impl<'a> GeneratorTransforms<'a> {
    pub fn new(generator: &'a mut TacGenerator) -> Self {
        Self { generator }
    }
}
