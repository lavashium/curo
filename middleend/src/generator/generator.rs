use super::transforms::*;
use accessors::accessors;
use constructors::constructors;
use language::*;
use common::*;
use zawarudo::zawarudo;
use crate::*;


#[accessors]
#[constructors]
pub struct TacGenerator<'scp> {
    pub program: &'scp mut TypedProgram,
}

impl<'scp> TacGenerator<'scp> {
    #[zawarudo(label = "Tac Generator")]
    pub fn generate(&mut self, ctx: &mut TacGenContext<'_, '_>) -> TacProgram {
        GeneratorTransforms::run(self.program, ctx)
    }
}
