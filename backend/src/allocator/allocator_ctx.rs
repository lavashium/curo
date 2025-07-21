use common::*;
use accessors::accessors;
use constructors::constructors;

use std::collections::HashMap;

#[accessors]
#[constructors]
pub struct AllocatorContext<'scp, 'ctx> {
    pub ctx: &'scp mut CompilerContext<'ctx>,
    pub stack_map: HashMap<String, i32>,
    pub next_offset: i32,
}