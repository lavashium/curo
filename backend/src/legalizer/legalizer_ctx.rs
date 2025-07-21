use common::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct LegalizerContext<'scp, 'ctx> {
    pub ctx: &'scp mut CompilerContext<'ctx>,
    pub stack_size: i32
}