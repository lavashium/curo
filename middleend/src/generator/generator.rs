use super::transforms::*;
use accessors::accessors;
use constructors::constructors;
use language::*;
use zawarudo::zawarudo;

use crate::*;

#[accessors]
#[constructors]
pub struct TacGenerator<'scp> {
    pub source_ast: &'scp mut AstProgram,
}

impl<'scp> TacGenerator<'scp> {
    #[zawarudo(label = "Tac Generator")]
    pub fn generate(&mut self, ctx: &'scp mut TacGenContext<'scp, '_>) -> TacProgram {
        GeneratorTransforms::new(ctx).transform_program(self.source_ast)
    }
}
