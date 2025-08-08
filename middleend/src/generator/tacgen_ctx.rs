use common::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct TacGenContext<'scp, 'ctx> {
    pub ctx: &'scp mut CompilerContext<'ctx>,
}

impl Context for TacGenContext<'_, '_> {}