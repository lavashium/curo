use common::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct GeneratorContext<'scp, 'ctx> {
    pub ctx: &'scp mut CompilerContext<'ctx>,
}

impl Context for GeneratorContext<'_, '_> {}