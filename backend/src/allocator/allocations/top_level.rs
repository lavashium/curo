use super::*;
use crate::asm::*;
use common::*;

impl<'scp, 'ctx> Factory<(), AsmTopLevel> for AllocatorAllocations<'scp, 'ctx> {
    fn run(top_level: &mut AsmTopLevel, ctx: &mut AllocatorContext<'scp, 'ctx>) {
        match top_level {
            AsmTopLevel::Function { identifier: _, global: _, instructions, stack_size } => {
                for instruction in instructions {
                    Self::run(instruction, ctx);
                }
                
                *stack_size = -ctx.next_offset;
            }
            AsmTopLevel::StaticVariable { identifier, global, init } => {}
        }

    }
}