use super::*;
use crate::asm::*;

pub struct MovLegalizer;

impl Legalizer for MovLegalizer {
    fn legalize(instr: &AsmInstruction) -> Option<Vec<AsmInstruction>> {
        match instr {
            AsmInstruction::Mov { src, dst } if both_stack_operands(&src, &dst) => Some(vec![
                AsmInstruction::new_mov(src.clone(), AsmOperand::Reg(AsmReg::R10)),
                AsmInstruction::new_mov(AsmOperand::Reg(AsmReg::R10), dst.clone()),
            ]),

            _ => None,
        }
    }
}
