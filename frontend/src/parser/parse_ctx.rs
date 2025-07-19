use common::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct ParserContext<'scp, 'ctx> {
    pub ctx: &'scp mut CompilerContext<'ctx>,
    pub min_prec: u8
}