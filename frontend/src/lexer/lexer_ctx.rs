use common::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct LexerContext<'scp, 'ctx> {
    pub ctx: &'scp mut CompilerContext<'ctx>,
}