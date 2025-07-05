use super::*;
use crate::asm::*;

pub struct CompareLegalizer;

impl Legalizer for CompareLegalizer {
    fn legalize(instr: &AsmInstruction) -> Option<Vec<AsmInstruction>> {
        match instr {
            AsmInstruction::Cmp { operand1, operand2 }
                if is_stack_operand(operand1) && is_stack_operand(operand2) =>
            {
                Some(vec![
                    AsmInstruction::new_mov(operand1.clone(), AsmOperand::new_reg(AsmReg::R10)),
                    AsmInstruction::new_cmp(AsmOperand::new_reg(AsmReg::R10), operand2.clone()),
                ])
            }

            AsmInstruction::Cmp { operand1, operand2 }
                if matches!(operand2, AsmOperand::Imm(_)) =>
            {
                Some(vec![
                    AsmInstruction::new_mov(operand2.clone(), AsmOperand::new_reg(AsmReg::R11)),
                    AsmInstruction::new_cmp(operand1.clone(), AsmOperand::new_reg(AsmReg::R11)),
                ])
            }

            _ => None,
        }
    }
}
