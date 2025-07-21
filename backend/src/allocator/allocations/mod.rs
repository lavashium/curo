mod program;
mod function;
mod instruction;

use crate::asm::*;
use super::*;
use constructors::constructors;

#[constructors]
pub struct AllocatorAllocations<'scp, 'ctx> {
    pub ctx: &'scp mut AllocatorContext<'scp, 'ctx>,
}

impl<'scp, 'ctx> AllocatorAllocations<'scp, 'ctx> {
    pub fn replace_operand(&mut self, operand: &mut AsmOperand) -> AsmOperand {
        match operand {
            AsmOperand::Pseudo(identifier) => {
                let offset = self.ctx.stack_map.entry(identifier.clone()).or_insert_with(|| {
                    let offset = self.ctx.next_offset;
                    self.ctx.next_offset -= 4;
                    offset
                });
                AsmOperand::new_stack(*offset)
            }
            other => other.clone(),
        }
    }
}