use super::*;
use crate::asm::*;
use common::*;

impl<'scp, 'ctx> Factory<(), AsmInstruction> for AllocatorAllocations<'scp, 'ctx> {
    fn run(instruction: &mut AsmInstruction, ctx: &mut AllocatorContext<'scp, 'ctx>) {
        match instruction {
            AsmInstruction::Mov { src, dst } => {
                *src = Self::replace_operand(src, ctx);
                *dst = Self::replace_operand(dst, ctx);
            }
            AsmInstruction::Unary { unary_operator: _, operand } => {
                *operand = Self::replace_operand(operand, ctx);
            }
            AsmInstruction::Binary { binary_operator: _, src, dst } => {
                *src = Self::replace_operand(src, ctx);
                *dst = Self::replace_operand(dst, ctx);
            }
            AsmInstruction::Idiv { operand } => {
                *operand = Self::replace_operand(operand, ctx);
            }
            AsmInstruction::Cmp { operand1, operand2 } => {
                *operand1 = Self::replace_operand(operand1, ctx);
                *operand2 = Self::replace_operand(operand2, ctx);
            }
            AsmInstruction::SetCC { cond: _, operand } => {
                *operand = Self::replace_operand(operand, ctx);
            }
            AsmInstruction::Push(asm_operand) => {
                *asm_operand = Self::replace_operand(asm_operand, ctx);
            }
            AsmInstruction::Ret => {}
            AsmInstruction::Cdq => {}
            AsmInstruction::AllocateStack(_amount) => {}
            AsmInstruction::Jmp(_label) => {}
            AsmInstruction::JmpCC { cond: _, label: _ } => {}
            AsmInstruction::Label(_name) => {}
            AsmInstruction::DeallocateStack(_value) => {}
            AsmInstruction::Call(_func) => {}
        }
    }
}