use crate::asm::*;
use super::allocations::*;
use super::allocator_ctx::*;
use common::*;
use accessors::accessors;
use zawarudo::zawarudo;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct AsmAllocator<'scp> {
    source_asm: &'scp mut AsmProgram,
}

impl<'scp> AsmAllocator<'scp> {
    #[zawarudo(label = "Register Allocator")]
    pub fn allocate(&mut self, ctx: &'scp mut AllocatorContext<'scp, '_>) {
        AllocatorAllocations::run(self.source_asm, ctx)
    }
}
