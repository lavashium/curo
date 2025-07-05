use super::transforms::*;
use accessors::accessors;
use constructors::constructors;
use language::*;

#[accessors]
#[constructors]
pub struct TacGenerator {
    pub tempgen: TempGen,
}

impl TacGenerator {
    pub fn parse(&mut self, program: AstProgram) -> TacProgram {
        GeneratorTransforms::new(self).transform_program(&program)
    }
}
