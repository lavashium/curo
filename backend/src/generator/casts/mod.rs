mod function;
mod instruction;
mod program;

pub use function::*;
pub use instruction::*;
pub use program::*;

use super::AsmGenerator;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
#[allow(dead_code)]
pub struct GeneratorCasts<'a> {
    generator: &'a mut AsmGenerator,
}
