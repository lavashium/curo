use super::*;
use crate::asm::*;
use common::*;

impl<'scp, 'ctx> AllocatorAllocations<'scp, 'ctx> {
    pub fn allocate_instruction(&mut self, instruction: &mut AsmInstruction) {
        <Self as Factory<(), Self, AsmInstruction>>::run(self, instruction)
    }
}

impl<'scp, 'ctx> Factory<(), Self, AsmInstruction> for AllocatorAllocations<'scp, 'ctx> {
    fn run(driver: &mut Self, instruction: &mut AsmInstruction) {
        match instruction {
            AsmInstruction::Mov { src, dst } => {
                *src = driver.replace_operand(src);
                *dst = driver.replace_operand(dst);
            }
            AsmInstruction::Unary { unary_operator: _, operand } => {
                *operand = driver.replace_operand(operand);
            }
            AsmInstruction::Binary { binary_operator: _, src, dst } => {
                *src = driver.replace_operand(src);
                *dst = driver.replace_operand(dst);
            }
            AsmInstruction::Idiv { operand } => {
                *operand = driver.replace_operand(operand);
            }
            AsmInstruction::Cmp { operand1, operand2 } => {
                *operand1 = driver.replace_operand(operand1);
                *operand2 = driver.replace_operand(operand2);
            }
            AsmInstruction::SetCC { cond: _, operand } => {
                *operand = driver.replace_operand(operand);
            }
            AsmInstruction::Push(asm_operand) => {
                *asm_operand = driver.replace_operand(asm_operand);
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