mod program;
mod top_level;
mod instruction;

use std::marker::PhantomData;

use crate::asm::*;
use super::*;
use common::*;

pub struct AllocatorAllocations<'scp, 'ctx> {
    _driver: PhantomData<AllocatorContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for AllocatorAllocations<'scp, 'ctx> {
    type Context = AllocatorContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> AllocatorAllocations<'scp, 'ctx> {
    pub fn replace_operand(operand: &mut AsmOperand, ctx: &mut AllocatorContext<'scp, 'ctx>) -> AsmOperand {
        match operand {
            AsmOperand::Pseudo(name) => {
                if let Some(symbol) = ctx.ctx.symtable.get(name) {
                    if let IdentifierAttrs::StaticAttr { .. } = symbol.attrs {
                        return AsmOperand::Data(name.clone());
                    }
                }
                
                if let Some(&offset) = ctx.stack_map.get(name) {
                    return AsmOperand::new_stack(offset);
                }
                
                let offset = {
                    ctx.next_offset -= 4;
                    ctx.next_offset
                };
                
                ctx.stack_map.insert(name.clone(), offset);
                AsmOperand::new_stack(offset)
            }
            other => other.clone(),
        }
    }
}