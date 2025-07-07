use super::transforms::*;
use accessors::accessors;
use constructors::constructors;
use language::*;
use zawarudo::zawarudo;

#[accessors]
#[constructors]
pub struct TacGenerator {
    pub tempgen: TempGen,
}

impl TacGenerator {
    #[zawarudo(label = "Tac Generator")]
    pub fn parse(&mut self, program: AstProgram) -> TacProgram {
        GeneratorTransforms::new(self).transform_program(&program)
    }
}
