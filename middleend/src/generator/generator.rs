use language::*;
use super::transforms::*;

pub struct TacGenerator {
   pub tempgen: TempGen,
}

impl TacGenerator {
    pub fn new() -> Self {
        Self {
            tempgen: TempGen::new(),
        }
    }

    pub fn parse(&mut self, program: AstProgram) -> TacProgram {
        GeneratorTransforms::new(self).transform_program(&program)
    }
}
