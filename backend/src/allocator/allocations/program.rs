use super::*;
use crate::asm::*;
use common::*;

impl Factory<(), AsmProgram, AllocatorContext<'_, '_>> for AllocatorAllocations {
    fn run(program: &mut AsmProgram, ctx: &mut AllocatorContext) {
        for function in program.function_definitions_mut() {
            Self::run(function, ctx);
            function.set_stack_size(-ctx.next_offset);
        }
    }

}