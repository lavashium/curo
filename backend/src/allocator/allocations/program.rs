use super::*;
use crate::asm::*;
use common::*;

impl<'scp, 'ctx> Factory<(), AsmProgram> for AllocatorAllocations<'scp, 'ctx> {
    fn run(program: &mut AsmProgram, ctx: &mut AllocatorContext<'scp, 'ctx>) {
        for top_level in program.top_level_mut() {
            Self::run(top_level, ctx);
        }
    }

}