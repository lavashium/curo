mod program;
mod function;
mod instruction;

use crate::asm::*;
use super::*;
use constructors::constructors;

#[constructors]
pub struct AllocatorAllocations;

impl AllocatorAllocations {
    pub fn replace_operand(operand: &mut AsmOperand, ctx: &mut AllocatorContext) -> AsmOperand {
        match operand {
            AsmOperand::Pseudo(identifier) => {
                let offset = ctx.stack_map.entry(identifier.clone()).or_insert_with(|| {
                    ctx.next_offset -= 4;
                    ctx.next_offset
                });
                AsmOperand::new_stack(*offset)
            }
            other => other.clone(),
        }
    }
}